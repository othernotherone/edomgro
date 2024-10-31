use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrgError {
    #[error("parsing error: {0}")]
    ParseError(String),
    
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type OrgResult<T> = Result<T, OrgError>; 