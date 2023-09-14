#![deny(clippy::all)]

use std::str::FromStr;

use ldk_node::generate_entropy_mnemonic;
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
    println!("IP{:}   ===>  Port {:}", ipv4, port);
    let addr = "sdf";
    Ok(NetAddress {
      inner: ldk_node::NetAddress::from_str(addr).unwrap(),
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
    println!("{:?}", &listening_address.inner);

    let config = ldk_node::Config {
      storage_dir_path: storage_dir_path,
      network: network.into(),
      listening_address: None,
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
  pub fn set_entropy_bip39_mnemonic(
    &mut self,
    mnemonic: String,
    passphrase: Option<String>,
  ) -> &Self {
    let seed = generate_entropy_mnemonic();
    self
      .inner
      .set_entropy_bip39_mnemonic(seed, Some(String::from("")));
    self
  }

  #[napi]
  pub fn set_esplora_server(&mut self, url: String) -> &Self {
    self.inner.set_esplora_server(url.to_string());
    self
  }

  #[napi]
  pub fn build(&mut self) -> Result<Node, Error> {
    Ok(Node {
      inner: self.inner.build().unwrap(),
    })
  }

  // #[napi]
  // pub fn from_config(&self, config: &Config) -> Result<(), Error> {
  //   ldk_node::Builder::from_config(config.unwrap());
  //   Ok(())
  // }
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
  pub fn node_id(&mut self) -> String {
    self.inner.node_id().to_string()
  }
}
