use thiserror::Error;

#[derive(Debug, Error)]
pub enum AOCError {
    #[error("Recap error : {_0}")]
    RecapError(#[from] recap::Error),
}
