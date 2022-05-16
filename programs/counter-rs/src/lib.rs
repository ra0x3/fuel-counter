#[macro_use]
extern crate log;

use fuels::prelude::*;
use fuels::signers::wallet::Wallet;
use fuels_abigen_macro::abigen;

pub async fn get_contract_id(provider: &Provider, wallet: &Wallet) -> String {
    dotenv::dotenv().ok();
    match dotenv::var("CONTRACT_ID") {
        Ok(id) => {
            debug!("Found contract ID located in .env");
            id
        }
        _ => {
            debug!("Creating new deployment for non-existent contract");
            let compiled =
                Contract::load_sway_contract("./../counter/out/debug/counter.bin").unwrap();

            let contract_id = Contract::deploy(&compiled, provider, wallet, tx_params())
                .await
                .unwrap();

            contract_id.to_string()
        }
    }
}

pub fn tx_params() -> TxParameters {
    let gas_price = 0;
    let gas_limit = 1_000_000;
    let byte_price = 0;
    TxParameters::new(Some(gas_price), Some(gas_limit), Some(byte_price), None)
}

abigen!(Counter, "./../counter/out/debug/counter-abi.json");
