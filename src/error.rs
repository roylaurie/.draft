use thiserror;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An unspecified error occured.")]
    Unspecified,
    #[error("{0}")]
    Generic(String),
    #[error("The Account could not be found: {0}")]
    AccountNotFound(String),
    #[error("The Account Definition could not be found: {0}")]
    DefinitionNotFound(String),
}

pub type Result<T> = ::core::result::Result<T, Error>;