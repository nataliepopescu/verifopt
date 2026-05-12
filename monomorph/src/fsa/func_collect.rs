use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::{CrateDefItems, DefId};
use rustc_public::ty::{AssocKind, GenericArgKind, ImplDef, ImplTrait, RigidTy, TyKind};

use rustc_public::Symbol;

use log::{debug, error, info, warn};

pub struct FuncVal {}

pub struct FuncMap {
    // HAD:
    // For CHA/RTA
    // - [x] trait_to_struct_impls
    // For intersecting trait impls w struct impls
    // - [ ] struct_to_impls
    // - [ ] impl_blocks_to_fn_impls
    // - [ ] trait_fn_impls
    // For getting a fn's trait
    // - [x] assoc_fns_to_trait

    // WANT:
    // - (CHA/RTA) HashMap<Trait, Vec<Struct>> (trait_to_struct_impls)
    // - HashMap<AssocFn, Trait> (assoc_fns_to_trait)
    // - HashMap<(Struct, Trait), FnImpls>

    // (CHA/RTA) HashMap<Trait, Vec<Struct>>
    pub trait_structs: HashMap<DefId, Vec<DefId>>,
    // HashMap<AssocFn, Trait>
    pub assoc_fn_traits: HashMap<DefId, DefId>,
    // HashMap<(Struct, Trait), FnImpls>
    // FIXME use Trait or use Container?
    // - using Trait will lead to the Value being a Vec (that needs to be iterated through)
    // - while using Container will lead to the Key pair only pointing to a single DefId (I _believe_)
    //   - but how do we get the Container when interpreting on-the-fly?
    pub struct_trait_fns: HashMap<(DefId, DefId), Vec<DefId>>,
}

impl FuncMap {
    pub fn new() -> FuncMap {
        Self {
            trait_structs: HashMap::default(),
            assoc_fn_traits: HashMap::default(),
            struct_trait_fns: HashMap::default(),
        }
    }
}

pub struct FuncCollectPass;

impl FuncCollectPass {
    pub fn new() -> FuncCollectPass {
        Self {}
    }

    pub fn run(&self, fmap: &mut FuncMap) {
        self.collect_metadata(fmap);
        debug!("fmap.trait_structs: {:?}", fmap.trait_structs);
        debug!("fmap.assoc_fn_traits: {:?}", fmap.assoc_fn_traits);
        debug!("fmap.struct_trait_fns: {:?}", fmap.struct_trait_fns);
    }

    fn collect_metadata(&self, fmap: &mut FuncMap) {
        for impl_def in rustc_public::all_trait_impls() {
            debug!("\n###################");

            let trait_impl = impl_def.trait_impl();

            // Get Trait DefId
            let trait_defid = trait_impl.value.def_id.0;
            debug!("trait_defid: {:?}", trait_defid);

            // Get Struct DefId
            let result = std::panic::catch_unwind(|| {
                self.get_struct_defid(&trait_impl)
            });
            if result.is_err() {
                debug!("CAUGHT PANIC");
                continue;
            }
            let struct_defid;
            if let Some(struct_defid_inner) = result.unwrap() {
                struct_defid = struct_defid_inner;
            } else {
                debug!("got a None struct_defid option (FIXME)");
                continue;
            }
            debug!("struct_defid: {:?}", struct_defid);

            // Get AssocFn DefIds
            let mut assoc_fn_defids = self.get_assoc_fn_defids(&impl_def);
            debug!("assoc_fn_defids: {:?}", assoc_fn_defids);
            if assoc_fn_defids.is_empty() {
                debug!("NO ASSOC FNS");
                continue;
            }

            // Add struct to list of structs that impl this trait
            match fmap.trait_structs.get_mut(&trait_defid) {
                Some(struct_vec) => struct_vec.push(struct_defid),
                None => {
                    fmap.trait_structs.insert(trait_defid, vec![struct_defid]);
                }
            }

            // Add back pointers from associated fns to this trait
            for assoc_fn_defid in &assoc_fn_defids {
                match fmap.assoc_fn_traits.get(&assoc_fn_defid) {
                    None => {
                        fmap.assoc_fn_traits.insert(*assoc_fn_defid, trait_defid);
                    }
                    Some(_) => panic!("already mapped this assoc fn to a trait"),
                }
            }

            // Add associated fns mapping to this struct/trait pair
            match fmap.struct_trait_fns.get_mut(&(struct_defid, trait_defid)) {
                Some(assoc_fn_vec) => assoc_fn_vec.append(&mut assoc_fn_defids),
                None => {
                    fmap.struct_trait_fns.insert((struct_defid, trait_defid), assoc_fn_defids);
                }
            }
        }
    }

    fn get_struct_defid(&self, trait_impl: &ImplTrait) -> Option<DefId> {
        // FIXME is there a better way to get Self type?
        //debug!("all genargs: {:?}", &trait_impl.value.args().0);
        let mut struct_defid = None;

        for genarg in &trait_impl.value.args().0 {
        //for (i, genarg) in trait_impl.value.args().0.clone().into_iter().enumerate() {
        //    debug!("genarg #{}", i);
            match genarg {
                GenericArgKind::Lifetime(region) => debug!("lifetime: {:?}", region),
                GenericArgKind::Type(ty) => {
                    //debug!("type: {:?}", ty);
                    //debug!("ty.kind: {:?}", ty.kind());
                    match ty.kind() {
                        TyKind::RigidTy(rigidty) => match rigidty {
                            // TODO process _adt_genargs
                            RigidTy::Adt(adtdef, adt_genargs) => {
                                debug!("ADT rigidty");
                                if !adt_genargs.0.is_empty() {
                                    warn!("process adt generic args: {:?}", adt_genargs);
                                }

                                match struct_defid {
                                    None => struct_defid = Some(adtdef.0),
                                    Some(existing_struct_defid) => {
                                        error!("already seen adt {:?} in this trait impls genargs", existing_struct_defid);
                                    }
                                }
                            }
                            // TODO
                            _ => warn!("other rigidty kind"),
                        }
                        // TODO
                        _ => warn!("other ty kind"),
                    }
                }
                GenericArgKind::Const(tyconst) => debug!("const: {:?}", tyconst),
            }
        }

        if struct_defid.is_none() {
            error!("never saw an Adt in this trait impls genargs");
        }

        struct_defid
    }

    fn get_assoc_fn_defids(&self, impl_def: &ImplDef) -> Vec<DefId> {
        let mut assoc_fns = Vec::new();

        for assoc_item in impl_def.associated_items() {
            match assoc_item.kind {
                AssocKind::Fn { name: _, has_self } => {
                    // if has_self is false, cannot be dynamically dispatched
                    if !has_self {
                        debug!("NO SELF");
                        continue;
                    }
                    assoc_fns.push(assoc_item.def_id.0);
                }
                // TODO
                _ => warn!("other assoc kind"),
            }
            info!("assoc_item container: {:?}", assoc_item.container)
        }

        assoc_fns
    }
}
