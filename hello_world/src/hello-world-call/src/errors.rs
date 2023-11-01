use candid::CandidType;
use flex_error::{define_error, TraceError};
use ic_agent::AgentError;
use serde::Deserialize;
use thiserror::Error;

define_error! {
    VpError {
        AgentError
            [TraceError<AgentError>]
            |_| { "vp agent error" },

        CustomError
            { reason: String }
            | e | { format!("Custom error: ({})", e.reason ) },

        DecodeIcTypeError
            [ TraceError<candid::Error>]
            | _ | { "decode ic type error" },

        PrincipalError
            [ TraceError<ic_agent::export::PrincipalError> ]
            | _ | { "build principal error"},

        CreateIdentityError
            [ TraceError<ic_agent::identity::PemError>]
            | _ | { "create identity failed" },
    }
}

#[derive(CandidType, Deserialize, Debug, Error)]
pub enum Error {
    #[error("dummy error: {0}")]
    DummyError(String),
}
