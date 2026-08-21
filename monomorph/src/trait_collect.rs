use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::ty::{
    AssocContainer, AssocKind, FnDef, GenericArgs, ImplDef, ImplTrait, RigidTy, TyKind,
};
use rustc_public::{CrateDefItems, DefId};

//use log::debug;

pub struct TraitVal {}

pub struct TraitStore {
    // HashMap<Struct, Vec<Trait>>
    pub struct_traits: HashMap<DefId, Vec<DefId>>,
    // (CHA/RTA) HashMap<Trait, Vec<(Struct, TraitRef's own GenericArgs)>>
    pub trait_structs: HashMap<DefId, Vec<(DefId, GenericArgs)>>,
    // HashMap<AssocFnDecl, Trait>
    pub assoc_fn_traits: HashMap<DefId, DefId>,
    // HashMap<(Struct, AssocFnDecl), Vec<AssocFnImpl>>
    pub struct_assoc_fns: HashMap<(DefId, DefId), Vec<DefId>>,
    // HashMap<Trait, Vec<AssocFnDecl>>
    pub trait_fns: HashMap<DefId, Vec<DefId>>,
    // HashMap<Trait, Vec<AssocFnImpl>>
    pub default_impls: HashMap<DefId, Vec<DefId>>,
}

impl TraitStore {
    pub fn new() -> TraitStore {
        Self {
            struct_traits: HashMap::default(),
            trait_structs: HashMap::default(),
            assoc_fn_traits: HashMap::default(),
            struct_assoc_fns: HashMap::default(),
            trait_fns: HashMap::default(),
            default_impls: HashMap::default(),
        }
    }
}

pub struct TraitCollectPass;

impl TraitCollectPass {
    pub fn new() -> TraitCollectPass {
        Self {}
    }

    pub fn run(&self, tstore: &mut TraitStore) {
        //debug!("\nDEFAULTS\n");
        self.collect_default_impls(tstore);
        //debug!("\nOTHER IMPLS\n");
        self.collect_rest_impls(tstore);
    }

    fn collect_default_impls(&self, tstore: &mut TraitStore) {
        for trait_def in rustc_public::all_trait_decls() {
            //debug!("\n###################");

            //debug!("trait_def: {:?}", trait_def);

            let mut trait_fns = Vec::new();
            let mut default_impls = Vec::new();
            for assoc_item in trait_def.associated_items() {
                //debug!("assoc_item: {:?}", assoc_item);

                if assoc_item.is_impl_trait_in_trait() {
                    //debug!("TODO nested trait impl");
                }

                match assoc_item.kind {
                    AssocKind::Fn { .. } => match assoc_item.container {
                        AssocContainer::Trait => {
                            trait_fns.push(assoc_item.def_id.0);

                            if FnDef(assoc_item.def_id.0).has_body() {
                                //debug!("found default impl {:?}", assoc_item.def_id.0);
                                default_impls.push(assoc_item.def_id.0);

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
                        _ => todo!("diff container"),
                    },
                    _ => {}
                }
            }

            match tstore.trait_fns.get(&trait_def.0) {
                Some(_) => panic!("already set fns for trait {:?}", &trait_def.0),
                None => {
                    if !trait_fns.is_empty() {
                        //debug!("storing trait fn decls: {:?}", trait_fns);
                        tstore.trait_fns.insert(trait_def.0, trait_fns);
                    }
                }
            }

            match tstore.default_impls.get(&trait_def.0) {
                Some(_) => panic!("already set defaults for trait {:?}", &trait_def.0),
                None => {
                    //debug!("storing default impls: {:?}", default_impls);
                    tstore.default_impls.insert(trait_def.0, default_impls);
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
            //debug!("TRAIT: {:?}", trait_defid);

            // Get AssocFn DefIds - operates on `impl_def` directly, not on
            // whatever `get_struct_defid` finds below, so this (and the
            // trait-level back-pointer registration it feeds) runs
            // unconditionally. A blanket impl (`impl<P> Trait for P where
            // ...`) has no single concrete Self type, so `get_struct_defid`
            // legitimately returns None for it below - but that impl still
            // provides real, concrete method bodies (monomorphized per
            // instantiation), and this trait-level bookkeeping doesn't need
            // to know which struct to record that.
            let mut assoc_fn_defids = self.get_assoc_fn_defids(&impl_def);
            //debug!("assoc_fn_defids: {:?}", assoc_fn_defids);
            //debug!("trait_fn_defids: {:?}", tstore.trait_fns.get(&trait_defid));
            //debug!(
            //    "trait defaults: {:#?}",
            //    tstore.default_impls.get(&trait_defid)
            //);

            // Fill in non-overriden default implementations
            match tstore.trait_fns.get(&trait_defid) {
                Some(trait_fns) => {
                    if assoc_fn_defids.len() != trait_fns.len() {
                        let defaults = tstore.default_impls.get(&trait_defid).unwrap();
                        let mut missing_impls = Vec::new();
                        let impls: Vec<DefId> = assoc_fn_defids
                            .clone()
                            .into_iter()
                            .map(|(_, x)| x)
                            .collect();
                        for trait_fn in trait_fns {
                            if !impls.contains(trait_fn) {
                                missing_impls.push(trait_fn);
                            }
                        }
                        //debug!("MISSING: {:?}", missing_impls);

                        for missing in &missing_impls {
                            //debug!("ADDING: {:?}", missing);
                            if defaults.contains(missing) {
                                assoc_fn_defids.push((**missing, **missing));
                            } else {
                                //debug!("NO DEFAULT!");
                            }
                        }
                    }
                }
                None => {}
            }

            // Add back pointers from associated fns to this trait. This must
            // cover *every* assoc fn decl in the trait, not just the
            // dynamically-dispatchable (has_self) subset in
            // `assoc_fn_defids` - `get_trait_defid` is also consulted for
            // no-`self` fns (constructors, conversions, `Step`'s internal
            // methods, etc.) reached via ordinary static/simulated calls, and
            // will panic if it's missing an entry.
            for assoc_fn_decl_defid in self.get_all_assoc_fn_decl_defids(&impl_def) {
                match tstore.assoc_fn_traits.get(&assoc_fn_decl_defid) {
                    None => {
                        tstore
                            .assoc_fn_traits
                            .insert(assoc_fn_decl_defid, trait_defid);
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

            // Get Struct DefId - only needed for the struct-specific
            // bookkeeping below; a blanket impl legitimately has none, in
            // which case we skip just that part, not the trait-level
            // registration above (already done, and independent of this).
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
            //debug!("STRUCT: {:?}", struct_defid);

            // Add trait to list of traits that this struct impls
            match tstore.struct_traits.get_mut(&struct_defid) {
                Some(trait_vec) => {
                    //debug!("adding trait to existing vec: {:?}", trait_vec);
                    trait_vec.push(trait_defid);
                }
                None => {
                    //debug!("init w trait");
                    tstore.struct_traits.insert(struct_defid, vec![trait_defid]);
                }
            }

            // Add struct to list of structs that impl this trait
            match tstore.trait_structs.get_mut(&trait_defid) {
                Some(struct_vec) => {
                    struct_vec.push((struct_defid, trait_impl.value.args().clone()));
                }
                None => {
                    tstore.trait_structs.insert(
                        trait_defid,
                        vec![(struct_defid, trait_impl.value.args().clone())],
                    );
                }
            }

            // Add assoc fn impl mapping to this (struct/assoc fn decl) pair
            for (assoc_fn_impl_defid, assoc_fn_decl_defid) in &assoc_fn_defids {
                match tstore
                    .struct_assoc_fns
                    .get_mut(&(struct_defid, *assoc_fn_decl_defid))
                {
                    Some(existing_impls) => {
                        //debug!("ADDING TO EXISTING: {:?}", assoc_fn_impl_defid);
                        // Skip duplicates
                        if !existing_impls.contains(assoc_fn_impl_defid) {
                            existing_impls.push(*assoc_fn_impl_defid);
                        }
                    }
                    None => {
                        //debug!("INITING WITH: {:?}", assoc_fn_impl_defid);
                        tstore.struct_assoc_fns.insert(
                            (struct_defid, *assoc_fn_decl_defid),
                            vec![*assoc_fn_impl_defid],
                        );
                    }
                }
            }
        }
    }

    /// The Self type of this trait impl, via `TraitRef::self_ty()` directly
    fn get_struct_defid(&self, trait_impl: &ImplTrait) -> Option<DefId> {
        match trait_impl.value.self_ty().kind() {
            TyKind::RigidTy(RigidTy::Adt(adtdef, _adt_genargs)) => Some(adtdef.0),
            _ => None,
        }
    }

    /// Returns a vector of (concrete_impl_defid, decl_defid), one for each associated fn
    /// that can be dynamically dispatched (i.e. takes `self`). Used to build
    /// `struct_assoc_fns`, the vtable-candidate map - non-`self` fns
    /// (constructors, conversions, etc.) can never be called through a `dyn
    /// Trait`, so they're correctly excluded here.
    ///
    /// NOTE: this is *not* a complete list of associated fns in the impl -
    /// see `get_all_assoc_fn_decl_defids` for that, used for `assoc_fn_traits`
    /// back-pointer registration, which needs every assoc fn regardless of
    /// dispatchability.
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
                    //debug!("IMPL DEFID: {:?}", assoc_item.def_id.0);
                    //debug!("SPAN: {:?}", assoc_item.def_id.span());
                    assoc_fns.push((assoc_item.def_id.0, assoc_def.0));
                }
                _ => {} //warn!("other container kind"),
            }
        }

        assoc_fns
    }

    /// Returns the trait-decl DefId for every associated fn in this impl,
    /// regardless of whether it takes `self`. Used only to populate
    /// `assoc_fn_traits` back-pointers, which `get_trait_defid` needs to be
    /// able to resolve *any* assoc fn - including no-`self` constructors and
    /// conversions like `Default::default`, `FromStr::from_str`,
    /// `From::from`/`TryFrom::try_from`, `FromIterator::from_iter`, and
    /// `Step::{steps_between, forward_checked, backward_checked, forward,
    /// backward}` - none of which can be dynamically dispatched, but all of
    /// which can still show up as the target of a static/simulated call
    /// during interpretation.
    fn get_all_assoc_fn_decl_defids(&self, impl_def: &ImplDef) -> Vec<DefId> {
        let mut decls = Vec::new();

        for assoc_item in impl_def.associated_items() {
            // Only Fn assoc items have a meaningful trait-decl DefId to
            // register here; assoc consts/types aren't looked up via
            // `assoc_fn_traits` (see `get_trait_defid`'s callers).
            match assoc_item.kind {
                AssocKind::Fn { .. } => {}
                _ => continue,
            }

            if let AssocContainer::TraitImpl(assoc_def) = assoc_item.container {
                decls.push(assoc_def.0);
            }
        }

        decls
    }
}
