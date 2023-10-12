import express from 'express'
import { LogLevel, Network, Config, Builder, NetAddress, Node, PublicKey, ChannelConfig } from '../'

const app: express.Application = express()
const port: number = 300
app.listen(port, () => {
  console.log(`TypeScript with Express http://localhost:${port}/`)
})

const wrapResponse = (res: any, content: any) => {
  res.send(`<pre>\n${content}\n</pre>`)
}

let storageDirPath = `temp/${Date.now()}`
let listeningAddress = new NetAddress('127.0.0.1', 2000)
const peerConfig = {
  node_id: '03f0b14fe8e1c5f651840c5be08e1a3c543422f3f8789947b6774c4f7e26e0570c',
  address: new NetAddress('127.0.0.1', 5001),
}

const config = new Config(storageDirPath, Network.Regtest, listeningAddress, 12, 12, 12, 12, LogLevel.Debug)
const builder = Builder.fromConfig(config)

builder.setEntropyBip39Mnemonic('awkward fox lawn senior flavor cook genuine cake endorse rare walk this')
builder.setEsploraServer('http://127.0.0.1:30000')

let node: Node
try {
  node = builder.build()
  console.log('Node Started ====>', node.start())
  console.log('Node Id ====>', node.nodeId())
  console.log('Listening Address ====>', node.listeningAddress())

  node.nextEvent().then((e) => console.log('Next event at JS====>', e))
  node.waitNextEvent().then((e) => console.log('Wait event at JS====>', e))
  node.eventHandled().then((e) => console.log('Event handled at JS====>', e))
} catch (e) {
  console.log('Build and start error', e)
}

app.get('/', (req, res) => {
  let response = ''
  response += 'Node Id: ' + node.nodeId()
  response += '\nListening Address: ' + node.listeningAddress()
  response += '\nFunding address: ' + JSON.stringify(node.newOnchainAddress(), undefined, 2)
  response += '\nSync: ' + node.syncWallets()
  response += '\nSpendable Balance: ' + node.spendableOnchainBalanceSats()
  response += '\nTotal Balance: ' + node.totalOnchainBalanceSats()

  let channels = node.listChannels()

  response += '\nChannels:' + JSON.stringify(channels, undefined, 2)

  wrapResponse(res, response)
})

app.get('/receive', (req, res) => {
  let response = node.receivePayment(777000, 'Demo Descrioptio', 478798)
  wrapResponse(res, response)
})

app.get('/send', (req, res) => {
  try {
    let invoice = `${req.query.invoice}`
    let response = node.sendPayment(invoice)
    // let response = node.sendPaymentUsingAmount(invoice, 12500)
    // let response = node.sendSpontaneousPayment(12500, new PublicKey(peerConfig.node_id))
    // let txid = node.sendToOnchainAddress({addressHex: 'bcrt1qrcl2q4sh2mvlzlq0rv2q8tnhwldd2sjyvj3lqe'}, 12500)
    // let response = node.sendAllToOnchainAddress({ addressHex: 'bcrt1qrcl2q4sh2mvlzlq0rv2q8tnhwldd2sjyvj3lqe' })
    wrapResponse(res, JSON.stringify(response))
  } catch (e) {
    console.log(e)
    wrapResponse(res, 'Failed')
  }
})

app.get('/open_channel', (req, res) => {
  try {
    let response = node.connectOpenChannel(
      new PublicKey(peerConfig.node_id),
      new NetAddress('127.0.0.1', 5001),
      888000,
      null,
      new ChannelConfig(12, 12, 12, 12, 12),
      false,
    )
    wrapResponse(res, response)
  } catch (e) {
    console.log(e)
    wrapResponse(res, 'Failed')
  }
})

app.get('/close_channel', (req, res) => {
  try {
    let channels = node.listChannels()
    let response = node.closeChannel(channels[0].channelId, new PublicKey(channels[0].counterpartyNodeId))

    wrapResponse(res, response)
  } catch (e) {
    console.log(e)
    wrapResponse(res, 'Failed')
  }
})

app.get('/payments', (req, res) => {
  try {
    let payments = node.listPayments()
    wrapResponse(res, JSON.stringify(payments))
  } catch (e) {
    console.log(e)
    wrapResponse(res, 'Failed')
  }
})

app.get('/misc', (req, res) => {
  try {
    let msg = [12, 15, 18, 15, 78, 56]
    let sign = node.signMessage(msg)
    let verify = node.verifySignature(msg, sign, new PublicKey(peerConfig.node_id))

    let channels = node.listChannels()
    let update_channel = node.updateChannelConfig(
      channels[0].channelId,
      new PublicKey(peerConfig.node_id),
      new ChannelConfig(15, 15, 15, 15, 15),
    )

    console.log('Channel config updated ====>', update_channel)
    wrapResponse(res, 'working...')
  } catch (e) {
    console.log(e)
    wrapResponse(res, 'Failed')
  }
})
