#![deny(clippy::all)]

use std::str::FromStr;

use ldk_node::bip39::Mnemonic;
use ldk_node::io::SqliteStore;
use napi::bindgen_prelude::Array;
use napi::bindgen_prelude::FromNapiValue;
use napi::bindgen_prelude::ToNapiValue;
use napi::Env;
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
pub struct PeerDetails {
  inner: ldk_node::PeerDetails,
}

impl PeerDetails {
  pub fn new(peer: ldk_node::PeerDetails) -> Self {
    PeerDetails { inner: peer }
  }
}

#[napi]
pub struct PublicKey {
  inner: ldk_node::bitcoin::secp256k1::PublicKey,
  pub node_id: String,
}

#[napi]
impl PublicKey {
  #[napi(constructor)]
  pub fn new(node_id: String) -> Result<Self, Error> {
    Ok(PublicKey {
      inner: ldk_node::bitcoin::secp256k1::PublicKey::from_str(&node_id).unwrap(),
      node_id: node_id,
    })
  }
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
  pub fn set_entropy_seed_path(&mut self, seed_path: String) -> Result<bool, Error> {
    self.inner.set_entropy_seed_path(seed_path);
    Ok(true)
  }

  #[napi]
  pub fn set_entropy_seed_bytes(&mut self, seed_bytes: Vec<u8>) -> Result<bool, Error> {
    let _ = self.inner.set_entropy_seed_bytes(seed_bytes);
    Ok(true)
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
  pub fn set_gossip_source_p2p(&mut self) -> Result<bool, Error> {
    self.inner.set_gossip_source_p2p();
    Ok(true)
  }

  #[napi]
  pub fn set_gossip_source_rgs(&mut self, rgs_server_url: String) -> Result<bool, Error> {
    self.inner.set_gossip_source_rgs(rgs_server_url);
    Ok(true)
  }

  #[napi]
  pub fn set_storage_dir_path(&mut self, storage_dir_path: String) -> Result<bool, Error> {
    self.inner.set_storage_dir_path(storage_dir_path);
    Ok(true)
  }

  #[napi]
  pub fn set_network(&mut self, network: Network) -> Result<bool, Error> {
    self.inner.set_network(network.into());
    Ok(true)
  }

  #[napi]
  pub fn set_listening_address(&mut self, listening_address: &NetAddress) -> Result<bool, Error> {
    self
      .inner
      .set_listening_address(listening_address.inner.to_owned());
    Ok(true)
  }

  #[napi]
  pub fn set_log_level(&mut self, level: LogLevel) -> Result<bool, Error> {
    self.inner.set_log_level(level.into());
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
  pub fn node_id(&mut self) -> Result<PublicKey, Error> {
    let node_id = self.inner.node_id().to_owned();
    Ok(PublicKey {
      inner: self.inner.node_id().to_owned(),
      node_id: node_id.to_string(),
    })
  }

  #[napi]
  pub fn listening_address(&mut self) -> Option<String> {
    let address = self.inner.listening_address();
    if address.is_none() {
      return None;
    } else {
      return Some(address.unwrap().to_owned().to_string());
    }
  }

  #[napi]
  pub fn new_onchain_address(&mut self) -> Result<String, Error> {
    let address = self.inner.new_onchain_address();
    Ok(address.unwrap().to_owned().to_string())
  }

  #[napi]
  pub fn spendable_onchain_balance_sats(&mut self) -> Result<u64, Error> {
    Ok(self.inner.spendable_onchain_balance_sats().unwrap())
  }

  #[napi]
  pub fn total_onchain_balance_sats(&mut self) -> Result<u64, Error> {
    Ok(self.inner.total_onchain_balance_sats().unwrap())
  }

  #[napi]
  pub fn receive_payment(
    &mut self,
    amount_msat: u32,
    description: String,
    expiry_secs: u32,
  ) -> Result<String, Error> {
    let desc = description.as_str();
    Ok(
      self
        .inner
        .receive_payment(u64::from(amount_msat), desc, expiry_secs)
        .unwrap()
        .to_string(),
    )
  }

  #[napi]
  pub fn receive_variable_amount_payment(
    &mut self,
    description: String,
    expiry_secs: u32,
  ) -> Result<String, Error> {
    let desc = description.as_str();
    Ok(
      self
        .inner
        .receive_variable_amount_payment(desc, expiry_secs)
        .unwrap()
        .to_string(),
    )
  }

  #[napi]
  pub fn connect(
    &mut self,
    node_id: &PublicKey,
    address: &NetAddress,
    persist: bool,
  ) -> Result<bool, Error> {
    let _ = self
      .inner
      .connect(node_id.inner.to_owned(), address.inner.to_owned(), persist);
    Ok(true)
  }

  #[napi]
  pub fn list_peers(&mut self, env: Env) -> Array {
    let mut arr = env.create_array(2).unwrap();

    arr.insert(1).unwrap();
    arr.insert("test").unwrap();

    arr

    // let peers = self.inner.list_peers();
    // let mut arr = env.create_array(peers.len() as u32).unwrap();
    // for peer in &peers {
    //   let detail = PeerDetails::new(peer.to_owned());

    //   arr.insert(detail).unwrap();
    // }

    // // println!("Array {:?}", arr.to_owned());
    // arr
  }

  #[napi]
  pub fn connect_open_channel(
    &mut self,
    node_id: &PublicKey,
    address: &NetAddress,
    channel_amount_sats: u32,
  ) -> Result<bool, Error> {
    let _ = self.inner.connect_open_channel(
      node_id.inner.to_owned(),
      address.inner.to_owned(),
      u64::from(channel_amount_sats),
      None,
      None,
      false,
    );

    print!("List channels==> {:?}", self.inner.list_channels());
    Ok(true)
  }
}
