extern crate polywrap_client;
extern crate safe_rust_playground;
extern crate serde;

use polywrap_client::msgpack::serialize;
use safe_rust_playground::{
    helpers::get_client, AddSignatureArgs, CreateTransactionArgs, EncodeAddOwner,
    ExecuteTransactionArgs, SafeTransaction, Transaction, TxReceipt, SAFE_ADDRESS,
    SAFE_MANAGER_URI,
};

const OWNER_TO_BE_ADDED: &str = "0x0Ce3cC862b26FC643aA8A73D2D30d47EF791941e";

fn main() {
    let client = get_client(None);
    let owners =
        client.invoke::<Vec<String>>(&SAFE_MANAGER_URI.clone(), "getOwners", None, None, None);

    if owners.is_err() {
        panic!("Error fetching owners")
    }

    println!("Current owners of safe: {:#?}", owners.unwrap());

    let add_owner_encoded = client.invoke::<String>(
        &SAFE_MANAGER_URI,
        "encodeAddOwnerWithThresholdData",
        Some(
            &serialize(&EncodeAddOwner {
                owner_address: String::from(OWNER_TO_BE_ADDED),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if add_owner_encoded.is_err() {
        panic!("Error encoding owner: {:?}", add_owner_encoded.unwrap_err())
    }

    println!(
        "Add owner encoded: {:?}",
        add_owner_encoded.clone().unwrap()
    );

    let transaction = Transaction {
        to: SAFE_ADDRESS.clone(),
        data: add_owner_encoded.unwrap(),
        value: String::from("0"),
    };

    println!("Transaction to be created: {:#?}", transaction);
    let create_transaction = client.invoke::<SafeTransaction>(
        &SAFE_MANAGER_URI,
        "createTransaction",
        Some(
            &serialize(&CreateTransactionArgs {
                tx: transaction.clone(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if create_transaction.is_err() {
        panic!(
            "Error creating transaction: {:?}",
            create_transaction.unwrap_err()
        )
    }

    println!(
        "Transaction created: {:#?}",
        create_transaction.clone().unwrap()
    );

    let sign_transaction = client.invoke::<SafeTransaction>(
        &SAFE_MANAGER_URI,
        "addSignature",
        Some(
            &serialize(&AddSignatureArgs {
                tx: create_transaction.unwrap(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if sign_transaction.is_err() {
        panic!(
            "Error signing transaction: {:?}",
            sign_transaction.unwrap_err()
        )
    }

    println!(
        "Transaction signed: {:#?}",
        sign_transaction.clone().unwrap()
    );

    let serialized_sign_transaction = serialize(&ExecuteTransactionArgs {
        tx: sign_transaction.clone().unwrap(),
    })
    .unwrap();
    println!("LA SERIALIZACION");
    dbg!(serialized_sign_transaction);

    let execute_transaction = client.invoke::<TxReceipt>(
        &SAFE_MANAGER_URI,
        "executeTransaction",
        Some(
            &serialize(&ExecuteTransactionArgs {
                tx: sign_transaction.unwrap(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if execute_transaction.is_err() {
        panic!(
            "Error executing transaction: {:?}",
            execute_transaction.unwrap_err()
        )
    }

    println!(
        "Transaction executed with hash: {:?}",
        execute_transaction.unwrap().transaction_hash
    );
    print!("Owner with address: {} has been added", OWNER_TO_BE_ADDED);
}
