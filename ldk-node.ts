import { LogLevel, Network, Config, Builder, NetAddress } from '.'

const config = new Config('temp', Network.Testnet, new NetAddress('127.0.0.1', 5000), 12, 12, 12, 12, LogLevel.Debug)

const builder = new Builder()

builder.setEntropyBip39Mnemonic('sfd')

const node = builder.build()
node.start()

console.log('====>', node.nodeId())
