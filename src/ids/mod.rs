use std::{io, fmt::Display};

use openssl::{error::ErrorStack, aes::KeyError};

use crate::{bags::BagError, apns::APNSError};
use thiserror::Error;

pub mod user;
pub mod signing;
pub mod identity;

#[derive(Error, Debug)]
pub enum IDSError {
    SSLError(ErrorStack),
    PlistError(plist::Error),
    RequestError(reqwest::Error),
    AuthError(plist::Value),
    BagError(BagError),
    CertError(plist::Dictionary),
    RegisterFailed(u64),
    IoError(io::Error),
    LookupFailed(u64),
    KeyError(KeyError),
    APNsError(APNSError),
    TwoFaError
}

impl Display for IDSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl From<APNSError> for IDSError {
    fn from(value: APNSError) -> Self {
        IDSError::APNsError(value)
    }
}

impl From<KeyError> for IDSError {
    fn from(value: KeyError) -> Self {
        IDSError::KeyError(value)
    }
}

impl From<io::Error> for IDSError {
    fn from(value: io::Error) -> Self {
        IDSError::IoError(value)
    }
}

impl From<BagError> for IDSError {
    fn from(value: BagError) -> Self {
        IDSError::BagError(value)
    }
}

impl From<ErrorStack> for IDSError {
    fn from(value: ErrorStack) -> Self {
        IDSError::SSLError(value)
    }
}

impl From<plist::Error> for IDSError {
    fn from(value: plist::Error) -> Self {
        IDSError::PlistError(value)
    }
}

impl From<reqwest::Error> for IDSError {
    fn from(value: reqwest::Error) -> Self {
        IDSError::RequestError(value)
    }
}