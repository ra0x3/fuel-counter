#[macro_use]
extern crate log;

use fuel_core::{
    chain_config::{ChainConfig, CoinConfig, StateConfig},
    service::{Config, DbType, FuelService},
};
use fuel_gql_client::client::FuelClient;
use fuels::{
    prelude::{Contract, LocalWallet, Provider, TxParameters, DEFAULT_COIN_AMOUNT},
    signers::wallet::Wallet,
    test_helpers::setup_address_and_coins,
};
use fuels_abigen_macro::abigen;
use std::net::{Ipv4Addr, SocketAddr};

pub async fn get_contract_id(wallet: &Wallet) -> String {
    dotenv::dotenv().ok();
    match dotenv::var("CONTRACT_ID") {
        Ok(id) => {
            debug!("Found contract ID located in .env");
            id
        }
        _ => {
            debug!("Creating new deployment for non-existent contract");
            let _compiled =
                Contract::load_sway_contract("./../counter/out/debug/counter.bin").unwrap();

            let bin_path = "./../counter/out/debug/counter.bin".to_string();
            let contract_id = Contract::deploy(&bin_path, wallet, tx_params())
                .await
                .unwrap();

            contract_id.to_string()
        }
    }
}

pub async fn setup_provider_and_wallet() -> (Provider, LocalWallet) {
    let (secret, coins) = setup_address_and_coins(1, DEFAULT_COIN_AMOUNT);

    let coin_configs = coins
        .into_iter()
        .map(|(utxo_id, coin)| CoinConfig {
            tx_id: Some(*utxo_id.tx_id()),
            output_index: Some(utxo_id.output_index() as u64),
            block_created: Some(coin.block_created),
            maturity: Some(coin.maturity),
            owner: coin.owner,
            amount: coin.amount,
            asset_id: coin.asset_id,
        })
        .collect();

    let config = Config {
        chain_conf: ChainConfig {
            initial_state: Some(StateConfig {
                coins: Some(coin_configs),
                ..StateConfig::default()
            }),
            ..ChainConfig::local_testnet()
        },
        database_type: DbType::InMemory,
        utxo_validation: false,
        addr: SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 4000),
        ..Config::local_node()
    };

    let srv = FuelService::new_node(config).await.unwrap();
    let client = FuelClient::from(srv.bound_address);
    info!("Fuel client started at {:?}", client);

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
