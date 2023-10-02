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

#[napi(object)]
#[derive(Debug)]
pub struct PeerDetails {
  pub node_id: String,
  pub address: String,
  pub is_persisted: bool,
  pub is_connected: bool,
}

impl PeerDetails {
  pub fn new(peer: ldk_node::PeerDetails) -> Self {
    PeerDetails {
      node_id: peer.node_id.to_string(),
      address: peer.address.to_string(),
      is_persisted: peer.is_persisted,
      is_connected: peer.is_connected,
    }
  }
}

#[napi]
#[derive(Debug, Clone, Copy)]
pub struct ChannelId {
  inner: [u8; 32],
}

#[napi]
impl From<ldk_node::ChannelId> for ChannelId {
  fn from(value: ldk_node::ChannelId) -> Self {
    ChannelId { inner: value.0 }
  }
}
#[napi]
impl From<ChannelId> for ldk_node::ChannelId {
  fn from(value: ChannelId) -> Self {
    ldk_node::ChannelId(value.inner)
  }
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct OutPoint {
  pub txid: String,
  pub vout: u32,
}

impl OutPoint {
  pub fn new(outpoint: Option<ldk_node::bitcoin::OutPoint>) -> Option<Self> {
    Some(OutPoint {
      txid: outpoint?.txid.to_string(),
      vout: outpoint?.vout,
    })
  }
}

#[napi(object)]
#[derive(Debug)]
pub struct UserChannelId {
  pub user_channel_id_hex: String,
}

#[napi(object)]
#[derive(Debug)]
pub struct ChannelDetails {
  // pub channel_id: ChannelId,
  pub counterparty_node_id: String,
  pub funding_txo: Option<OutPoint>,
  pub channel_value_sats: u32,
  pub unspendable_punishment_reserve: Option<u32>,
  //   pub user_channel_id: UserChannelId,
  pub feerate_sat_per_1000_weight: u32,
  pub balance_msat: u32,
  pub outbound_capacity_msat: u32,
  pub inbound_capacity_msat: u32,
  pub confirmations_required: Option<u32>,
  pub confirmations: Option<u32>,
  pub is_outbound: bool,
  pub is_channel_ready: bool,
  pub is_usable: bool,
  pub is_public: bool,
  pub cltv_expiry_delta: Option<u16>,
}

impl ChannelDetails {
  pub fn new(channel: ldk_node::ChannelDetails) -> Self {
    let punishment = channel.unspendable_punishment_reserve;
    let punishment_value;
    if punishment.is_none() {
      punishment_value = None;
    } else {
      punishment_value = Some(u32::from(punishment.unwrap() as u32));
    };

    let ch_id = channel.channel_id;
    println!("Channel Id:: {:?}", ch_id);
    let converted = ChannelId { inner: ch_id.0 };

    ChannelDetails {
      // channel_id: ChannelId::from(channel.channel_id),
      counterparty_node_id: channel.counterparty_node_id.to_string(),
      funding_txo: OutPoint::new(channel.funding_txo),
      channel_value_sats: channel.channel_value_sats as u32,
      unspendable_punishment_reserve: punishment_value,
      //   user_channel_id: channel.user_channel_id.into(),
      feerate_sat_per_1000_weight: channel.feerate_sat_per_1000_weight,
      balance_msat: channel.balance_msat as u32,
      outbound_capacity_msat: channel.outbound_capacity_msat as u32,
      inbound_capacity_msat: channel.inbound_capacity_msat as u32,
      confirmations_required: channel.confirmations_required,
      confirmations: channel.confirmations,
      is_outbound: channel.is_outbound,
      is_channel_ready: channel.is_channel_ready,
      is_usable: channel.is_usable,
      is_public: channel.is_public,
      cltv_expiry_delta: channel.cltv_expiry_delta,
    }
  }
}

pub fn node_error(e: ldk_node::NodeError) -> napi::Error {
  Error::new(napi::Status::GenericFailure, e.to_string())
}

pub fn build_error(e: ldk_node::BuildError) -> napi::Error {
  Error::new(napi::Status::GenericFailure, e.to_string())
}
