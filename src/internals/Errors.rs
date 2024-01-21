use thiserror::Error;
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("error type 1")]
    error1
}