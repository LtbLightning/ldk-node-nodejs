/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum Network {
  Bitcoin = 0,
  Regtest = 1,
  Signet = 2,
  Testnet = 3,
}
export const enum LogLevel {
  Gossip = 0,
  Trace = 1,
  Debug = 2,
  Warn = 3,
  Info = 4,
  Error = 5,
}
export interface PeerDetails {
  nodeId: string
  address: string
  isPersisted: boolean
  isConnected: boolean
}
export interface OutPoint {
  txid: string
  vout: number
}
export interface UserChannelId {
  userChannelIdHex: string
}
export interface ChannelDetails {
  counterpartyNodeId: string
  fundingTxo?: OutPoint
  channelValueSats: number
  unspendablePunishmentReserve?: number
  feerateSatPer1000Weight: number
  balanceMsat: number
  outboundCapacityMsat: number
  inboundCapacityMsat: number
  confirmationsRequired?: number
  confirmations?: number
  isOutbound: boolean
  isChannelReady: boolean
  isUsable: boolean
  isPublic: boolean
  cltvExpiryDelta?: number
}
export class ChannelId {}
export class NetAddress {
  constructor(ipv4: string, port: number)
}
export class PublicKey {
  constructor(nodeId: string)
}
export class Config {
  constructor(
    storageDirPath: string,
    network: Network,
    listeningAddress: NetAddress,
    defaultCltvExpiryDelta: number,
    onchainWalletSyncIntervalSecs: number,
    walletSyncIntervalSecs: number,
    feeRateCacheUpdateIntervalSecs: number,
    logLevel: LogLevel,
  )
}
export class Builder {
  constructor()
  static fromConfig(config: Config): Builder
  setEntropySeedPath(seedPath: string): boolean
  setEntropySeedBytes(seedBytes: Array<number>): boolean
  setEntropyBip39Mnemonic(mnemonic: string, passphrase?: string | undefined | null): boolean
  setEsploraServer(url: string): boolean
  setGossipSourceP2P(): boolean
  setGossipSourceRgs(rgsServerUrl: string): boolean
  setStorageDirPath(storageDirPath: string): boolean
  setNetwork(network: Network): boolean
  setListeningAddress(listeningAddress: NetAddress): boolean
  setLogLevel(level: LogLevel): boolean
  build(): Node
}
export class Node {
  start(): boolean
  stop(): boolean
  syncWallets(): boolean
  nodeId(): string
  listeningAddress(): string | null
  newOnchainAddress(): string
  spendableOnchainBalanceSats(): number
  totalOnchainBalanceSats(): number
  receivePayment(amountMsat: number, description: string, expirySecs: number): string
  receiveVariableAmountPayment(description: string, expirySecs: number): string
  connect(nodeId: PublicKey, address: NetAddress, persist: boolean): boolean
  listPeers(): Array<PeerDetails>
  connectOpenChannel(nodeId: PublicKey, address: NetAddress, channelAmountSats: number): boolean
  listChannels(): Array<ChannelDetails>
  sendPayment(invoice: string): PaymentHash
}
export class PaymentHash {}
