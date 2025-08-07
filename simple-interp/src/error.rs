use crate::statement::RVal;

use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Error {
    #[error("Scope {0} is undefined.")]
    UndefinedScope(&'static str),
    #[error("Symbol {0} is undefined.")]
    UndefinedSymbol(&'static str),
    #[error("Trait {0} is undefined.")]
    UndefinedTrait(&'static str),

    #[error("Symbol {0} already exists.")]
    SymbolAlreadyExists(&'static str),

    #[error("Scope backpointers differ.")]
    BackpointersDiffer(),
    #[error("{0} cannot be compared to {1}.")]
    IncomparableTypes(RVal, RVal),
    #[error("VarTypes cannot be compared.")]
    IncomparableVarTypes(),
    #[error("Inconsistent return types.")]
    InconsistentReturnTypes(),
    #[error("Attempting to merge constraints of different types.")]
    TypesDiffer(),

    #[error("{0} is not a function.")]
    NotAFunction(&'static str),
    #[error("{0} is not a scope.")]
    NotAScope(&'static str),
    #[error("{0} is not a trait.")]
    NotATrait(&'static str),

    #[error("Cannot assign var to a return value of None.")]
    CannotAssignNoneRetval(),
    #[error("Invalid RVal for Assignment.")]
    InvalidAssignmentRVal(),
    #[error("Switching on a function pointer, not a value.")]
    NoSwitchOnFuncPtr(),
    #[error("Unexpected scope.")]
    UnexpectedScope(),
    #[error("Cannot perform merge on Vec with no elements.")]
    VecSize(),
}
