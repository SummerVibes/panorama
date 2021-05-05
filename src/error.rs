//!error module
use thiserror::Error;

#[derive(Error,Debug)]
pub enum Error{
    #[error("Get Device from str error")]
    GetDeviceError,
    #[error("No Such Service")]
    NoSuchService
}
