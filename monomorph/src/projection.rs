use rustc_public::DefId;
use rustc_public::mir::{FieldIdx, Place, ProjectionElem};
use rustc_public::ty::{Ty, VariantIdx};
use rustc_public_bridge::IndexedVal;

use crate::constraints::unique_push;
use crate::constraints::{Constraints, VORval};

use log::{debug, error};

fn is_box(defid: &DefId) -> bool {
    let idx = defid.to_index();
    // FIXME why multiple defids for box?
    if idx == 18965 || idx == 18969 {
        return true;
    }
    false
}

pub struct ProjectionHandler;

impl ProjectionHandler {
    pub fn new() -> ProjectionHandler {
        Self {}
    }

    pub fn apply_projection(
        &self,
        backup_ty_naive: &Ty,
        resolved_ty: &Ty,
        constraints: Constraints,
        place: &Place,
    ) -> Constraints {
        if place.projection.is_empty() {
            return constraints;
        }

        debug!("APPLYING PROJECTIONS");

        let mut converted_projections = Vec::new();
        for constraint in constraints {
            match constraint {
                // If constraint is an IdkType or wider, just skip to the resolved_ty
                VORval::IdkType(_) | VORval::Idk => {
                    unique_push(&mut converted_projections, VORval::IdkType(*resolved_ty));
                }
                // Otherwise, try to process projections in order
                _ => {
                    let mut updated_constraint = constraint;
                    for projection in &place.projection {
                        // Iteratively apply projections on constraint each constraint
                        updated_constraint = self.apply_projection_helper(
                            backup_ty_naive,
                            resolved_ty,
                            &updated_constraint,
                            projection,
                        );
                        debug!("UPDATED CONSTRAINT: {:?}", updated_constraint);

                        // If constraint is an IdkType or wider, just skip to the resolved_ty
                        match updated_constraint {
                            VORval::IdkType(_) | VORval::Idk => {
                                unique_push(
                                    &mut converted_projections,
                                    VORval::IdkType(*resolved_ty),
                                );
                                break;
                            }
                            _ => {}
                        }
                    }
                    unique_push(&mut converted_projections, updated_constraint);
                }
            }
        }

        converted_projections
    }

    fn apply_projection_helper(
        &self,
        backup_ty_naive: &Ty,
        resolved_ty: &Ty,
        constraint: &VORval,
        projection: &ProjectionElem,
    ) -> VORval {
        debug!("APPLYING SINGLE PROJECTION TO VORVAL");
        debug!("backup_ty_naive: \n{:?}", backup_ty_naive);
        debug!("resolved_ty: \n{:?}", resolved_ty);
        debug!("constraint: {:?}", constraint);
        debug!("projection: {:?}", projection);

        match projection {
            ProjectionElem::Deref => self.apply_deref(constraint),
            ProjectionElem::Downcast(vidx) => self.apply_downcast(constraint, *vidx),
            ProjectionElem::Field(fidx, ty) => self.apply_field(constraint, fidx, ty),
            _ => todo!("handle projection: {:?}", projection),
        }
    }

    fn apply_deref(&self, constraint: &VORval) -> VORval {
        match constraint {
            VORval::AddressOf(inner) | VORval::Ref(inner) | VORval::RawPtr(inner) => *inner.clone(),
            VORval::IdkAdt(adtdef, genargs) => {
                debug!("adtdef: {:?}", adtdef);
                debug!("genargs: {:?}", genargs);
                if is_box(&adtdef.0) {
                    let genargs = genargs.clone().unwrap();
                    if genargs.len() > 1 {
                        error!("more than 1 genarg in box");
                    }
                    genargs[0].clone()
                } else {
                    todo!();
                }
            }
            _ => todo!(),
        }
    }

    fn apply_downcast(&self, constraint: &VORval, vidx: VariantIdx) -> VORval {
        match constraint {
            VORval::IdkAdt(def, genargs) => {
                debug!("def: {:?}", def);
                debug!("genargs: {:?}", genargs);
                debug!("vidx: {:?}", vidx);
                //debug!("variant: {:?}", def.variant(vidx).unwrap().name());
                //debug!("fields: {:?}", def.variant(vidx).unwrap().fields());

                constraint.clone()
                //match genargs {
                //    // FIXME this is wrong
                //    Some(genargs) => genargs.list[0].clone(),
                //    None => panic!("no genargs to idx into"),
                //}
            }
            _ => todo!(),
        }
    }

    fn apply_field(&self, constraint: &VORval, fidx: &FieldIdx, ty: &Ty) -> VORval {
        match constraint {
            VORval::Tuple(inner_vec) => inner_vec[*fidx].clone(),
            // FIXME widening to type, but can maybe retain info
            VORval::IdkAdt(_def, _genargs) => VORval::IdkType(*ty),
            _ => todo!(),
        }
    }
}
