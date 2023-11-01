pub mod errors;
pub mod identity;
pub mod types;

use crate::errors::Error;
use crate::errors::VpError;
use crate::identity::create_identity;
use candid::Principal;
use candid::{Decode, Encode};
use core::ops::Deref;
use ic_agent::agent::QueryBuilder;
use ic_agent::Agent;
use std::path::PathBuf;

#[derive(Debug)]
pub struct VpClient {
    pub agent: Agent,
}

impl VpClient {
    const LOCAL_NET: &'static str = "http://localhost:4943";
    #[allow(dead_code)]
    const MAIN_NET: &'static str = "https://ic0.app";

    pub async fn new(ic_endpoint_url: &str, pem_file: PathBuf) -> Result<Self, VpError> {
        let agent = Agent::builder()
            .with_url(ic_endpoint_url)
            .with_identity(create_identity(&pem_file).map_err(VpError::create_identity_error)?)
            .build()
            .map_err(VpError::agent_error)?;

        if ic_endpoint_url == Self::LOCAL_NET {
            agent.fetch_root_key().await.map_err(VpError::agent_error)?;
        }

        Ok(VpClient { agent })
    }

    async fn query_response(
        &self,
        canister_id: &str,
        method_name: &str,
        args: String,
    ) -> Result<Vec<u8>, VpError> {
        let canister_id = Principal::from_text(canister_id).map_err(VpError::principal_error)?;

        let response = QueryBuilder::new(&self.agent, canister_id, method_name.into())
            .with_arg(Encode!(&args).map_err(VpError::decode_ic_type_error)?)
            .call()
            .await
            .map_err(|e| {
                println!("query_ic: {:?}", e);
                VpError::agent_error(e)
            })?;
        Ok(response)
    }

    pub async fn call_greet(&self, canister_id: &str, msg: String) -> Result<String, VpError> {
        let result = self.query_response(canister_id, "greet", msg).await?;
        Decode!(result.as_slice(), String).map_err(|e| VpError::custom_error(e.to_string()))
    }

    pub async fn call_result_error(
        &self,
        canister_id: &str,
        mag: String,
    ) -> Result<String, VpError> {
        let result = self
            .query_response(canister_id, "result_error", mag)
            .await?;
        let result = Decode!(result.as_slice(), Result<String, Error>)
            .map_err(|e| VpError::custom_error(e.to_string()))?;
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(VpError::custom_error(e.to_string())),
        }
    }
}

impl Deref for VpClient {
    type Target = Agent;
    fn deref(&self) -> &Agent {
        &self.agent
    }
}

#[tokio::main]
async fn main() {
    let ic_client = VpClient::new(
        "http://localhost:4943",
        "/Users/davirain/.config/dfx/identity/default/identity.pem".into(),
    )
    .await
    .unwrap();
    let result = ic_client
        .call_greet("bkyz2-fmaaa-aaaaa-qaaaq-cai", "hello".into())
        .await
        .unwrap();
    println!("result is {result:?}");

    let result = ic_client
        .call_result_error("bkyz2-fmaaa-aaaaa-qaaaq-cai", "hello".into())
        .await;
    println!("result is {result:?}");
}
