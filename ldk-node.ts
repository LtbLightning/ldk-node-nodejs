import { LogLevel, Network, Config, Builder, NetAddress } from '.'

const main = () => {
  try {
    const config = new Config(
      'temp1',
      Network.Testnet,
      new NetAddress('127.0.0.1', 5000),
      12,
      12,
      12,
      12,
      LogLevel.Debug,
    )

    const builder = Builder.fromConfig(config)

    builder.setEntropyBip39Mnemonic('sfd')

    const node = builder.build()

    console.log('Node Started ====>', node.start())
    console.log('Node Id ====>', node.nodeId())
    console.log('Listening Address ====>', node.listeningAddress())
  } catch (e) {
    console.log('Rust Panic error', e, e.stackTrace)
  }
}

main()
