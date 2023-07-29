use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CommonError {
    #[error("Invalid seed: {seed}")]
    InvalidSeed { seed: String },
}
