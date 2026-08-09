use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Error {
    #[error("depth limit ({0}) reached while recursing")]
    RecurseLimit(u32),
    #[error("parametric summary build hit control flow driven by a parameter's value")]
    SummaryImprecise,
}
