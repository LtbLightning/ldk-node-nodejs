#![deny(clippy::all)]

use std::str::FromStr;

use ldk_node::bip39::Mnemonic;
use ldk_node::io::SqliteStore;
use napi::bindgen_prelude::FromNapiValue;
use napi::bindgen_prelude::ToNapiValue;
use napi::Error;
use napi_derive::napi;

#[napi]
pub enum Network {
  Bitcoin,
  Regtest,
  Signet,
  Testnet,
}

impl From<Network> for ldk_node::bitcoin::Network {
  fn from(item: Network) -> Self {
    match item {
      Network::Bitcoin => ldk_node::bitcoin::Network::Bitcoin,
      Network::Regtest => ldk_node::bitcoin::Network::Regtest,
      Network::Signet => ldk_node::bitcoin::Network::Signet,
      Network::Testnet => ldk_node::bitcoin::Network::Testnet,
    }
  }
}

#[napi]
pub enum LogLevel {
  Gossip,
  Trace,
  Debug,
  Warn,
  Info,
  Error,
}

impl From<LogLevel> for ldk_node::LogLevel {
  fn from(item: LogLevel) -> Self {
    match item {
      LogLevel::Gossip => ldk_node::LogLevel::Gossip,
      LogLevel::Trace => ldk_node::LogLevel::Trace,
      LogLevel::Debug => ldk_node::LogLevel::Debug,
      LogLevel::Warn => ldk_node::LogLevel::Warn,
      LogLevel::Info => ldk_node::LogLevel::Info,
      LogLevel::Error => ldk_node::LogLevel::Error,
    }
  }
}

#[napi]
pub struct NetAddress {
  inner: ldk_node::NetAddress,
}

#[napi]
impl NetAddress {
  #[napi(constructor)]
  pub fn new(ipv4: String, port: u32) -> Result<Self, Error> {
    let addr = format!("{}:{}", ipv4, port).to_string();
    Ok(NetAddress {
      inner: ldk_node::NetAddress::from_str(&addr).unwrap(),
    })
  }
}

#[napi]
pub struct Config {
  inner: ldk_node::Config,
}

#[napi]
impl Config {
  #[napi(constructor)]
  pub fn new(
    storage_dir_path: String,
    network: Network,
    listening_address: &NetAddress,
    default_cltv_expiry_delta: u32,
    onchain_wallet_sync_interval_secs: u32,
    wallet_sync_interval_secs: u32,
    fee_rate_cache_update_interval_secs: u32,
    log_level: LogLevel,
  ) -> Result<Self, Error> {
    let config = ldk_node::Config {
      storage_dir_path: storage_dir_path,
      network: network.into(),
      listening_address: Some(listening_address.inner.to_owned()),
      default_cltv_expiry_delta: default_cltv_expiry_delta,
      onchain_wallet_sync_interval_secs: u64::from(onchain_wallet_sync_interval_secs),
      wallet_sync_interval_secs: u64::from(wallet_sync_interval_secs),
      fee_rate_cache_update_interval_secs: u64::from(fee_rate_cache_update_interval_secs),
      trusted_peers_0conf: Vec::new(),
      log_level: log_level.into(),
    };
    Ok(Config { inner: config })
  }
}

#[napi]
pub struct Builder {
  inner: ldk_node::Builder,
}

#[napi]
impl Builder {
  #[napi(constructor)]
  pub fn new() -> Self {
    Builder {
      inner: ldk_node::Builder::new(),
    }
  }

  #[napi]
  pub fn from_config(config: &Config) -> Self {
    Builder {
      inner: ldk_node::Builder::from_config(config.inner.to_owned()),
    }
  }

  #[napi]
  pub fn set_entropy_bip39_mnemonic(
    &mut self,
    mnemonic: String,
    passphrase: Option<String>,
  ) -> Result<bool, Error> {
    let mnemonic_seed = Mnemonic::from_str(&mnemonic).unwrap();
    let password = Some(if passphrase != None {
      passphrase.unwrap()
    } else {
      String::from("")
    });
    self
      .inner
      .set_entropy_bip39_mnemonic(mnemonic_seed, password);
    Ok(true)
  }

  #[napi]
  pub fn set_esplora_server(&mut self, url: String) -> Result<bool, Error> {
    self.inner.set_esplora_server(url.to_string());
    Ok(true)
  }

  #[napi]
  pub fn build(&mut self) -> Result<Node, Error> {
    Ok(Node {
      inner: self.inner.build().unwrap(),
    })
  }
}

#[napi]
pub struct Node {
  inner: ldk_node::Node<SqliteStore>,
}

#[napi]
impl Node {
  #[napi]
  pub fn start(&mut self) -> Result<bool, Error> {
    let _ = self.inner.start();
    Ok(true)
  }

  #[napi]
  pub fn stop(&mut self) -> Result<bool, Error> {
    let _ = self.inner.stop();
    Ok(true)
  }

  #[napi]
  pub fn sync_wallets(&mut self) -> Result<bool, Error> {
    let _ = self.inner.sync_wallets();
    Ok(true)
  }

  #[napi]
  pub fn node_id(&mut self) -> String {
    self.inner.node_id().to_string()
  }

  #[napi]
  pub fn listening_address(&mut self) -> String {
    self
      .inner
      .listening_address()
      .unwrap()
      .to_owned()
      .to_string()
  }

  #[napi]
  pub fn new_onchain_address(&mut self) -> String {
    let address = self.inner.new_onchain_address();
    address.unwrap().to_owned().to_string()
  }

  #[napi]
  pub fn spendable_onchain_balance_sats(&mut self) -> Result<u64, Error> {
    Ok(self.inner.spendable_onchain_balance_sats().unwrap())
  }

  #[napi]
  pub fn total_onchain_balance_sats(&mut self) -> Result<u64, Error> {
    Ok(self.inner.total_onchain_balance_sats().unwrap())
  }
}
