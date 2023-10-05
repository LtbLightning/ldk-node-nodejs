use napi::bindgen_prelude::FromNapiValue;
use napi::bindgen_prelude::ToNapiValue;
use napi::Error;
use napi_derive::napi;

// use crate::get_payment_hash;
// use crate::PaymentHash;

#[napi(string_enum)]
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

#[napi(string_enum)]
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

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ChannelId {
  pub channel_id_hex: Vec<u8>,
}

impl ChannelId {
  pub fn from_ldk_node(value: ldk_node::ChannelId) -> Self {
    ChannelId {
      channel_id_hex: value.0.to_vec(),
    }
  }

  pub fn from_nodejs(channel_id: ChannelId) -> ldk_node::ChannelId {
    ldk_node::ChannelId(channel_id.channel_id_hex.to_owned().try_into().unwrap())
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
#[derive(Debug, Clone)]
pub struct UserChannelId {
  pub user_channel_id_hex: String,
}

#[napi(object)]
#[derive(Debug)]
pub struct ChannelDetails {
  pub channel_id: ChannelId,
  pub counterparty_node_id: String,
  pub funding_txo: Option<OutPoint>,
  pub channel_value_sats: u32,
  pub unspendable_punishment_reserve: Option<u32>,
  pub user_channel_id: UserChannelId,
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

    ChannelDetails {
      channel_id: ChannelId::from_ldk_node(channel.channel_id),
      counterparty_node_id: channel.counterparty_node_id.to_string(),
      funding_txo: OutPoint::new(channel.funding_txo),
      channel_value_sats: channel.channel_value_sats as u32,
      unspendable_punishment_reserve: punishment_value,
      user_channel_id: UserChannelId {
        user_channel_id_hex: channel.user_channel_id.0.to_string(),
      },
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

#[napi(object)]
#[derive(Debug, Clone)]
pub struct PaymentPreimage {
  pub field0: Vec<u8>,
}

impl PaymentPreimage {
  pub fn from(value: ldk_node::lightning::ln::PaymentPreimage) -> Self {
    PaymentPreimage {
      field0: value.0.to_vec(),
    }
  }
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct PaymentSecret {
  pub field0: Vec<u8>,
}

impl PaymentSecret {
  pub fn from(value: ldk_node::lightning::ln::PaymentSecret) -> Self {
    PaymentSecret {
      field0: value.0.to_vec(),
    }
  }
}

#[napi(object)]
#[derive(Debug)]
pub struct PaymentDetails {
  /// The payment hash, i.e., the hash of the `preimage`.
  pub hash: PaymentHash,
  /// The pre-image used by the payment.
  pub preimage: Option<PaymentPreimage>,
  /// The secret used by the payment.
  pub secret: Option<PaymentSecret>,
  /// The amount transferred.
  pub amount_msat: Option<u32>,
  /// The direction of the payment.
  pub direction: PaymentDirection,
  /// The status of the payment.
  pub status: PaymentStatus,
}

impl PaymentDetails {
  pub fn new(payment: ldk_node::PaymentDetails) -> Self {
    let amount_value;
    if payment.amount_msat.is_none() {
      amount_value = None;
    } else {
      amount_value = Some(payment.amount_msat.unwrap() as u32);
    };

    let pre_image;
    if payment.preimage.is_none() {
      pre_image = None;
    } else {
      pre_image = Some(PaymentPreimage::from(payment.preimage.unwrap()));
    }

    let secret;
    if payment.secret.is_none() {
      secret = None;
    } else {
      secret = Some(PaymentSecret::from(payment.secret.unwrap()));
    }

    PaymentDetails {
      hash: PaymentHash::from_ldk_node(payment.hash),
      preimage: pre_image,
      secret: secret,
      amount_msat: amount_value,
      direction: payment.direction.into(),
      status: payment.status.into(),
    }
  }
}

/// Represents the current status of a payment.
///
#[napi(string_enum)]
#[derive(Debug, PartialEq, Eq)]
pub enum PaymentStatus {
  /// The payment is still pending.
  Pending,
  /// The payment suceeded.
  Succeeded,
  /// The payment failed.
  Failed,
}

impl From<ldk_node::PaymentStatus> for PaymentStatus {
  fn from(value: ldk_node::PaymentStatus) -> Self {
    match value {
      ldk_node::PaymentStatus::Pending => PaymentStatus::Pending,
      ldk_node::PaymentStatus::Succeeded => PaymentStatus::Succeeded,
      ldk_node::PaymentStatus::Failed => PaymentStatus::Failed,
    }
  }
}

/// Represents the direction of a payment.
///
#[napi(string_enum)]
#[derive(Debug, PartialEq, Eq)]
pub enum PaymentDirection {
  /// The payment is inbound.
  Inbound,
  /// The payment is outbound.
  Outbound,
}

impl From<ldk_node::PaymentDirection> for PaymentDirection {
  fn from(value: ldk_node::PaymentDirection) -> Self {
    match value {
      ldk_node::PaymentDirection::Inbound => PaymentDirection::Inbound,
      ldk_node::PaymentDirection::Outbound => PaymentDirection::Outbound,
    }
  }
}

impl From<PaymentDirection> for ldk_node::PaymentDirection {
  fn from(value: PaymentDirection) -> Self {
    match value {
      PaymentDirection::Inbound => ldk_node::PaymentDirection::Inbound,
      PaymentDirection::Outbound => ldk_node::PaymentDirection::Outbound,
    }
  }
}

#[napi(object)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentHash {
  pub field0: Vec<u8>,
}

impl PaymentHash {
  pub fn from_ldk_node(hash: ldk_node::lightning::ln::PaymentHash) -> PaymentHash {
    PaymentHash {
      field0: hash.0.to_vec(),
    }
  }

  pub fn from_nodejs(hash: PaymentHash) -> ldk_node::lightning::ln::PaymentHash {
    ldk_node::lightning::ln::PaymentHash(hash.field0.to_owned().try_into().unwrap())
  }
}
