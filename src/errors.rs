use thiserror::Error;


#[derive(Clone, Copy, Error, Debug)]
pub enum Errors {
    #[error("Data too long")]
    RequestTooLong,

    #[error("Invalid request")]
    InvalidRequest,

    #[error("Session with this id already exist")]
    SessionAlreadyExist
}