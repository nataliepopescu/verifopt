use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Error {
    #[error("{0} dummy")]
    DummyErr(&'static str)
}
