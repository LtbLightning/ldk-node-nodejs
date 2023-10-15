import { LogLevel, Network, Config, Builder, NetAddress, Node, PublicKey, ChannelConfig } from '.'

let storageDirPath = `node_backup`
let logDirPath = `${storageDirPath}/logs`

const main = () => {
  try {
    let listeningAddress = new NetAddress('127.0.0.1', 2000)
    const peerConfig = {
      node_id: '03f0b14fe8e1c5f651840c5be08e1a3c543422f3f8789947b6774c4f7e26e0570c',
      address: new NetAddress('127.0.0.1', 5001),
    }

    // const config = new Config(
    //   storageDirPath,
    //   logDirPath,
    //   Network.Regtest,
    //   listeningAddress,
    //   12,
    //   12,
    //   12,
    //   12,
    //   12,
    //   LogLevel.Debug,
    // )

    const builder = new Builder()

    builder.setEntropyBip39Mnemonic('awkward fox lawn senior flavor cook genuine cake endorse rare walk this')
    builder.setEsploraServer('http://127.0.0.1:30000')
    builder.setNetwork(Network.Regtest)
    builder.setStorageDirPath(storageDirPath)
    builder.setLogLevel(LogLevel.Debug)
    builder.setListeningAddress(listeningAddress)

    const node: Node = builder.build()

    console.log('Node Started ====>', node.start())
    console.log('Node Id ====>', node.nodeId())
    console.log('Listening Address ====>', node.listeningAddress())
    console.log('Funding Address ====>', node.newOnchainAddress())
    // console.log('Sync ====>', node.syncWallets())
    // console.log('Spendable Balance ====>', node.spendableOnchainBalanceSats())
    // console.log('Total Balance ====>', node.totalOnchainBalanceSats())

    // // console.log('Receive payment ====>', node.receivePayment(45200, 'Invoice', 1520))
    // // console.log('Receive variable ====>', node.receiveVariableAmountPayment('Varaible amount invoice', 15200))

    // const peerConnected = node.connect(new PublicKey(peerConfig.node_id), peerConfig.address, true)
    // console.log('Connected Peer ====>', peerConnected)
    // console.log('Peers ====>', node.listPeers())

    // const openChannel = node.connectOpenChannel(
    //   new PublicKey(peerConfig.node_id),
    //   new NetAddress('127.0.0.1', 5001),
    //   888000,
    //   null,
    //   new ChannelConfig(12, 12, 12, 12, 12),
    //   false,
    // )
    // console.log('Opened channel ====>', openChannel)
    // console.log('channels ====>', node.listChannels())
  } catch (e) {
    console.log('Rust Panic error', e, e.stackTrace)
  }
}

main()
