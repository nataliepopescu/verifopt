//use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_public::mir::{Operand, Place, Rvalue};

use crate::trait_collect::TraitStore;
use crate::constraints::{Constraints, MapKey, MapValue, ScopeId};
use crate::InterpStore;

use log::debug;

pub struct RvalConverter<'a> {
    pub tstore: &'a TraitStore,
}

impl<'a> RvalConverter<'a> {
    pub fn new(tstore: &'a TraitStore) -> RvalConverter<'a> {
        Self { tstore }
    }

    pub fn convert(
        &self,
        istore: &InterpStore,
        cur_scope: ScopeId,
        to_convert: &Rvalue,
    ) -> Constraints {
        debug!("CONVERTING RVALUE");
        match to_convert {
            Rvalue::Use(op) => {
                debug!("USE");
                debug!("op: {:?}", op);
                self.convert_op(istore, cur_scope, op)
            },
            _ => todo!("other rval: {:?}", to_convert),
        }
    }

    fn convert_op(&self, istore: &InterpStore, cur_scope: ScopeId, op: &Operand) -> Constraints {
        debug!("CONVERTING OP");
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                self.convert_place(istore, cur_scope, place)
            }
            _ => todo!("other op: {:?}", op),
        }
    }

    fn convert_place(&self, istore: &InterpStore, cur_scope: ScopeId, place: &Place) -> Constraints {
        debug!("CONVERTING PLACE");
        if !place.projection.is_empty() {
            self.convert_projection(place);
        }

        match istore.scoped_get(
            cur_scope,
            &MapKey::Place(place.clone()),
        ) {
            Some(val) => match val {
                MapValue::Constraints(constraints) => {
                    return constraints;
                }
                _ => panic!("value should not be a scope"),
            }
            None => todo!("place has not been set, use backup type"),
        }
    }

    fn convert_projection(&self, _place: &Place) {
        todo!("handle projections");
    }
}
