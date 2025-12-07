use thiserror::Error;

#[repr(C)]
#[derive(Clone, Copy, Error, Debug)]
pub enum Errors {
    #[error("Unsupported")]
    Unsupported = 1,

    #[error("Data too long")]
    RequestTooLong,

    #[error("Invalid request")]
    InvalidRequest,

    #[error("Session with this id already exist")]
    SessionAlreadyExist,
}
