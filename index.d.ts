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
export class NetAddress {
  constructor(ipv4: string, port: number)
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
  static fromConfig(config: Config): this
  setEntropyBip39Mnemonic(mnemonic: string, passphrase?: string | undefined | null): this
  setEsploraServer(url: string): this
  build(): Node
}
export class Node {
  start(): boolean
  nodeId(): string
  listeningAddress(): void
}
