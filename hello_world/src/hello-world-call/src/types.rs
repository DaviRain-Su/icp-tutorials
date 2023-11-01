use crate::errors::VpError;
use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub enum VecResult {
    Ok(Vec<u8>),
    Err(String),
}

impl VecResult {
    pub fn transfer_anyhow(self) -> Result<Vec<u8>, VpError> {
        match self {
            VecResult::Ok(value) => Ok(value),
            VecResult::Err(e) => Err(VpError::custom_error(e)),
        }
    }
}
