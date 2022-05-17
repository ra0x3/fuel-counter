#[macro_use]
extern crate log;

use fuel_gql_client::client::FuelClient;
use fuel_core::{service::{Config, FuelService}};
use fuels::{
    prelude::{Contract, LocalWallet, Provider, TxParameters},
    signers::{fuel_crypto::SecretKey, wallet::Wallet},
};
use fuels_abigen_macro::abigen;
use rand::{rngs::StdRng, RngCore, SeedableRng};

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

pub async fn setup_provider_and_wallet() -> (Provider, LocalWallet) {
    let srv = FuelService::new_node(Config::local_node()).await.unwrap();
    let client = FuelClient::from(srv.bound_address);
    info!("Fuel client started at {:?}", client);

    let mut rng = StdRng::seed_from_u64(2322u64);
    let mut secret_seed = [0u8; 32];
    rng.fill_bytes(&mut secret_seed);
    let secret = unsafe { SecretKey::from_bytes_unchecked(secret_seed) };

    let provider = Provider::new(client);
    let wallet = LocalWallet::new_from_private_key(secret, provider.clone());

    (provider, wallet)
}

pub fn tx_params() -> TxParameters {
    let gas_price = 0;
    let gas_limit = 1_000_000;
    let byte_price = 0;
    TxParameters::new(Some(gas_price), Some(gas_limit), Some(byte_price), None)
}

abigen!(Counter, "./../counter/out/debug/counter-abi.json");
