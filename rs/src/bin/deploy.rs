extern crate polywrap_client;
extern crate safe_rust_playground;
extern crate serde;

use polywrap_client::msgpack::serialize;
use safe_rust_playground::{
    constants::ETHERS_CORE_WRAPPER_URI, helpers::get_client, SAFE_FACTORY_URI,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Connection {
    #[serde(rename = "networkNameOrChainId")]
    network_name_or_chain_id: Option<String>,
    node: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct AccountConfig {
    owners: Vec<String>,
    threshold: u32,
}

#[derive(Serialize, Deserialize)]
struct DeploymentConfig {
    #[serde(rename = "saltNonce")]
    salt_nonce: String,
}

#[derive(Serialize, Deserialize)]
struct DeploymentInput {
    #[serde(rename = "safeAccountConfig")]
    safe_account_config: AccountConfig,
    #[serde(rename = "safeDeploymentConfig")]
    safe_deployment_config: DeploymentConfig,
    connection: Option<Connection>,
}

#[derive(Serialize, Deserialize)]
struct DeploymentArgs {
    input: DeploymentInput,
}

fn main() {
    let client = get_client(None);

    let signer_address = client.invoke::<String>(
        &ETHERS_CORE_WRAPPER_URI.clone(),
        "getSignerAddress",
        None,
        None,
        None,
    );

    if signer_address.is_err() {
        panic!("Error fetching signer address")
    }

    println!("Address of the signer: {}", signer_address.clone().unwrap());

    let deployment_input = DeploymentArgs {
        input: DeploymentInput {
            safe_account_config: AccountConfig {
                owners: vec![signer_address.unwrap()],
                threshold: 1,
            },
            safe_deployment_config: DeploymentConfig {
                salt_nonce: "0x23423".to_string(),
            },
            connection: Some(Connection {
                network_name_or_chain_id: Some("goerli".to_string()),
                node: None,
            }),
        },
    };

    let expected_safe_address = client.invoke::<String>(
        &SAFE_FACTORY_URI.clone(),
        "predictSafeAddress",
        Some(&serialize(&deployment_input).unwrap()),
        None,
        None,
    );

    if expected_safe_address.is_err() {
        panic!(
            "Error predicting safe address: {}",
            expected_safe_address.unwrap_err().to_string()
        )
    }
    println!("Expected safe address: {}", expected_safe_address.unwrap());

    let deploy_safe = client.invoke::<String>(
        &SAFE_FACTORY_URI.clone(),
        "deploySafe",
        Some(&serialize(&deployment_input).unwrap()),
        None,
        None,
    );

    if deploy_safe.is_err() {
        panic!(
            "Error deploying safe: {}",
            deploy_safe.unwrap_err().to_string()
        )
    }
    println!("Safe deployed with hash: {}", deploy_safe.unwrap());
}
