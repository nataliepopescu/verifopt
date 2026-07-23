use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Error {
    #[error("depth limit ({0}) reached while recursing")]
    RecurseLimit(u32),
}
