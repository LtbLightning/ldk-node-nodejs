import { LogLevel, Network, Config, Builder, NetAddress, Node } from '.'

const main = () => {
  try {
    const config = new Config(
      `temp/${Date.now()}`,
      Network.Regtest,
      new NetAddress('127.0.0.1', 5000),
      12,
      12,
      12,
      12,
      LogLevel.Debug,
    )

    const builder = Builder.fromConfig(config)

    builder.setEntropyBip39Mnemonic('awkward fox lawn senior flavor cook genuine cake endorse rare walk this')
    builder.setEsploraServer('http://127.0.0.1:30000')

    const node: Node = builder.build()

    console.log('Node Started ====>', node.start())
    console.log('Node Id ====>', node.nodeId())
    console.log('Listening Address ====>', node.listeningAddress())
    console.log('Funding Address ====>', node.newOnchainAddress())
    console.log('Sync ====>', node.syncWallets())
    console.log('Spendable Balance ====>', node.spendableOnchainBalanceSats())
    console.log('Total Balance ====>', node.totalOnchainBalanceSats())
  } catch (e) {
    console.log('Rust Panic error', e, e.stackTrace)
  }
}

main()
