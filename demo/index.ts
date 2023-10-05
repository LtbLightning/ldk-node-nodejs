import express from 'express'
import { LogLevel, Network, Config, Builder, NetAddress, Node, PublicKey, ChannelId } from '../'

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

const node: Node = builder.build()
console.log('Node Started ====>', node.start())
console.log('Node Id ====>', node.nodeId())
console.log('Listening Address ====>', node.listeningAddress())

app.get('/', (req, res) => {
  let response = ''
  response += 'Node Id: ' + node.nodeId()
  response += '\nListening Address: ' + node.listeningAddress()
  response += '\nFunding address: ' + node.newOnchainAddress()
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
    wrapResponse(res, JSON.stringify(response))
  } catch (e) {
    console.log(e)
    wrapResponse(res, 'Failed')
  }
})

app.get('/open_channel', (req, res) => {
  try {
    let response = node.connectOpenChannel(new PublicKey(peerConfig.node_id), new NetAddress('127.0.0.1', 5001), 888000)
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
