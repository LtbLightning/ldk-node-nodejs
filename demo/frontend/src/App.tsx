import axios from 'axios'
import './App.css'
import { useEffect, useState } from 'react'

let Api = axios.create({ baseURL: 'http://localhost:4200/', withCredentials: false })

export const App = () => {
  const [nodeInfo, setNodeInfo] = useState<{ nodeId: string; listeningAddress?: string }>({
    nodeId: '',
    listeningAddress: '',
  })

  const [invoice, setInvoice] = useState('')
  const [amount, setAmount] = useState('')
  const [response, setResponse] = useState('')

  useEffect(() => {
    ;(async () => {
      let { data } = await Api.get('node-info')
      setNodeInfo(data)
    })()
  }, [])

  const send = async () => {
    let { data } = await Api.get(`send/?invoice=${invoice}`)
    setInvoice('')
    setResponse(JSON.stringify(data))
  }

  const receive = async () => {
    setAmount('')
    let { data } = await Api.get(`receive/?amount=${amount}`)
    setResponse(data)
  }
  return (
    <div className="container">
      <h1>Bitcoin Lightning Node Info</h1>
      <div id="nodeInfo">
        <NodeInfo title="Node Id" value={nodeInfo.nodeId} />
        <NodeInfo title="Listening Address" value={nodeInfo.listeningAddress} />
      </div>
      <div className="invoice-section">
        <h2>Pay Invoice</h2>
        <textarea
          placeholder="Paste your payment invoice here..."
          onChange={(e) => setInvoice(e.target.value)}
          value={invoice}
        />
        <br />
        <button className="pay-btn" onClick={send}>
          Pay Invoice
        </button>
      </div>
      <div className="invoice-section">
        <h2>Generate Invoice</h2>
        <input
          type="number"
          value={amount}
          placeholder="Enter amount in msats e.g. 10000"
          onChange={(e) => setAmount(e.target.value)}
        />
        <br />
        <button className="generate-btn" onClick={receive}>
          Generate Invoice
        </button>
      </div>

      <div className="invoice-section">
        <p>{response}</p>
      </div>
    </div>
  )
}

const NodeInfo = ({ title, value }: any) => (
  <div>
    <b>{title}: </b>
    {value}
  </div>
)
