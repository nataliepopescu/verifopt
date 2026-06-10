use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::ty::{
    AssocContainer, AssocKind, FnDef, GenericArgKind, ImplDef, ImplTrait, RigidTy, TyKind,
};
use rustc_public::{CrateDefItems, DefId};

//use log::debug;

pub struct TraitVal {}

pub struct TraitStore {
    // HashMap<Struct, Vec<Trait>>
    pub struct_traits: HashMap<DefId, Vec<DefId>>,
    // (CHA/RTA) HashMap<Trait, Vec<Struct>>
    pub trait_structs: HashMap<DefId, Vec<DefId>>,
    // HashMap<AssocFnDecl, Trait>
    pub assoc_fn_traits: HashMap<DefId, DefId>,
    // HashMap<(Struct, AssocFnDecl), Vec<AssocFnImpl>>
    pub struct_assoc_fns: HashMap<(DefId, DefId), Vec<DefId>>,
}

impl TraitStore {
    pub fn new() -> TraitStore {
        Self {
            struct_traits: HashMap::default(),
            trait_structs: HashMap::default(),
            assoc_fn_traits: HashMap::default(),
            struct_assoc_fns: HashMap::default(),
        }
    }
}

pub struct TraitCollectPass;

impl TraitCollectPass {
    pub fn new() -> TraitCollectPass {
        Self {}
    }

    pub fn run(&self, tstore: &mut TraitStore) {
        //debug!("DEFAULTS");
        self.collect_default_impls(tstore);
        //debug!("OTHER IMPLS");
        self.collect_rest_impls(tstore);
    }

    fn collect_default_impls(&self, tstore: &mut TraitStore) {
        for trait_def in rustc_public::all_trait_decls() {
            //debug!("\n###################");

            //debug!("trait_def: {:?}", trait_def);

            for assoc_item in trait_def.associated_items() {
                //debug!("assoc_item: {:?}", assoc_item);

                if assoc_item.is_impl_trait_in_trait() {
                    //debug!("TODO nested trait impl");
                }

                match assoc_item.kind {
                    AssocKind::Fn { .. } => match assoc_item.container {
                        AssocContainer::Trait => {
                            if FnDef(assoc_item.def_id.0).has_body() {
                                //debug!("found default impl {:?}", assoc_item.def_id.0);
                                match tstore.assoc_fn_traits.get_mut(&assoc_item.def_id.0) {
                                    Some(trait_defid) => {
                                        if *trait_defid != trait_def.0 {
                                            panic!("same assoc fn for multiple traits");
                                        }
                                    }
                                    None => {
                                        tstore
                                            .assoc_fn_traits
                                            .insert(assoc_item.def_id.0, trait_def.0);
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    fn collect_rest_impls(&self, tstore: &mut TraitStore) {
        for impl_def in rustc_public::all_trait_impls() {
            //debug!("\n###################");

            let trait_impl = impl_def.trait_impl();

            // Get Trait DefId
            let trait_defid = trait_impl.value.def_id.0;

            // Get Struct DefId
            let result = std::panic::catch_unwind(|| self.get_struct_defid(&trait_impl));
            if result.is_err() {
                continue;
            }
            let struct_defid;
            if let Some(struct_defid_inner) = result.unwrap() {
                struct_defid = struct_defid_inner;
            } else {
                //debug!("got a None struct_defid option (FIXME)");
                continue;
            }

            // Add trait to list of traits that this struct impls
            match tstore.struct_traits.get_mut(&struct_defid) {
                Some(trait_vec) => {
                    trait_vec.push(trait_defid);
                }
                None => {
                    tstore.struct_traits.insert(struct_defid, vec![trait_defid]);
                }
            }

            // Add struct to list of structs that impl this trait
            match tstore.trait_structs.get_mut(&trait_defid) {
                Some(struct_vec) => struct_vec.push(struct_defid),
                None => {
                    tstore.trait_structs.insert(trait_defid, vec![struct_defid]);
                }
            }

            // Get AssocFn DefIds
            let assoc_fn_defids = self.get_assoc_fn_defids(&impl_def);
            //debug!("assoc_fn_defids: {:?}", assoc_fn_defids);

            // Add back pointers from associated fns to this trait
            for (_, assoc_fn_decl_defid) in &assoc_fn_defids {
                match tstore.assoc_fn_traits.get(&assoc_fn_decl_defid) {
                    None => {
                        tstore
                            .assoc_fn_traits
                            .insert(*assoc_fn_decl_defid, trait_defid);
                    }
                    Some(existing_trait_defid) => {
                        if *existing_trait_defid != trait_defid {
                            panic!(
                                "already mapped this assoc fn to another trait: \n\texisting: {:?}\n\tcurrent: {:?}",
                                existing_trait_defid, trait_defid
                            );
                        }
                    }
                }
            }

            // Add assoc fn impl mapping to this (struct/assoc fn decl) pair
            for (assoc_fn_impl_defid, assoc_fn_decl_defid) in &assoc_fn_defids {
                match tstore
                    .struct_assoc_fns
                    .get_mut(&(struct_defid, *assoc_fn_decl_defid))
                {
                    Some(existing_impls) => {
                        // Skip duplicates
                        if !existing_impls.contains(assoc_fn_impl_defid) {
                            existing_impls.push(*assoc_fn_impl_defid);
                        }
                    }
                    None => {
                        tstore.struct_assoc_fns.insert(
                            (struct_defid, *assoc_fn_decl_defid),
                            vec![*assoc_fn_impl_defid],
                        );
                    }
                }
            }
        }
    }

    /// Proxies the Self defid for this implementation as the first ADT encountered in the genargs
    /// [This is probably wrong, needs fixing]
    fn get_struct_defid(&self, trait_impl: &ImplTrait) -> Option<DefId> {
        // FIXME is there a better way to get Self type?
        //debug!("all genargs: {:?}", &trait_impl.value.args().0);
        let mut struct_defid = None;

        for genarg in &trait_impl.value.args().0 {
            //for (i, genarg) in trait_impl.value.args().0.clone().into_iter().enumerate() {
            //    debug!("genarg #{}", i);
            match genarg {
                GenericArgKind::Lifetime(_region) => {} //debug!("lifetime: {:?}", region),
                GenericArgKind::Type(ty) => {
                    //debug!("type: {:?}", ty);
                    //debug!("ty.kind: {:?}", ty.kind());
                    match ty.kind() {
                        TyKind::RigidTy(rigidty) => match rigidty {
                            // TODO process _adt_genargs
                            RigidTy::Adt(adtdef, adt_genargs) => {
                                //debug!("ADT rigidty");
                                if !adt_genargs.0.is_empty() {
                                    //warn!("process adt generic args: {:?}", adt_genargs);
                                }

                                match struct_defid {
                                    None => struct_defid = Some(adtdef.0),
                                    Some(_existing_struct_defid) => {
                                        //error!(
                                        //    "already seen adt {:?} in this trait impls genargs",
                                        //    existing_struct_defid
                                        //);
                                    }
                                }
                            }
                            // TODO
                            _ => {} //warn!("other rigidty kind"),
                        },
                        // TODO
                        _ => {} //warn!("other ty kind"),
                    }
                }
                GenericArgKind::Const(_tyconst) => {} //debug!("const: {:?}", tyconst),
            }
        }

        if struct_defid.is_none() {
            //error!("never saw an Adt in this trait impls genargs");
        }

        struct_defid
    }

    /// Returns a vector of (concrete_impl_defid, decl_defid), one for each associated fn
    fn get_assoc_fn_defids(&self, impl_def: &ImplDef) -> Vec<(DefId, DefId)> {
        let mut assoc_fns = Vec::new();

        for assoc_item in impl_def.associated_items() {
            // If this assoc_item is not a function, skip
            match assoc_item.kind {
                AssocKind::Fn { name: _, has_self } => {
                    // If has_self is false, cannot be dynamically dispatched, so no need to store
                    if !has_self {
                        //debug!("NO SELF");
                        continue;
                    }
                }
                // TODO
                _ => {
                    //warn!("other assoc kind");
                    continue;
                }
            }

            //info!("assoc_item container: {:?}", assoc_item.container);
            match assoc_item.container {
                AssocContainer::TraitImpl(assoc_def) => {
                    assoc_fns.push((assoc_item.def_id.0, assoc_def.0));
                }
                _ => {} //warn!("other container kind"),
            }
        }

        assoc_fns
    }
}
