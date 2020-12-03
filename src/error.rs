use thiserror::Error;

#[derive(Debug, Error)]
pub enum AOCError {
    #[error("Recap error : {_0}")]
    RecapError(#[from] recap::Error),
    #[error("Error while parsing grid: {_0}")]
    GridError(#[from] crate::utils::grid::GridParsingError),
}
