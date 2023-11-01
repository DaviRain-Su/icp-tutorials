use candid::CandidType;
use serde::Deserialize;
use thiserror::Error;

#[derive(CandidType, Deserialize, Debug, Error)]
enum Error {
    #[error("dummy error: {0}")]
    DummyError(String),
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn result_error(name: String) -> Result<String, Error> {
    Err(Error::DummyError(name))
}
