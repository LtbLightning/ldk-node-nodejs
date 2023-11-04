## LDK-Node for NodeJs

<p>
  <a href="https://github.com/LtbLightning/ldk-node-nodejs/blob/HEAD/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="LDK-Node-NodeJs RN is released under the MIT license." />
  </a>
  <a href="https://github.com/LtbLightning/ldk-node-nodejs/blob/master/README.md">
    <img src="https://img.shields.io/badge/docs-red.svg" alt="Docs" />
  </a>
  <a href="https://www.npmjs.com/package/ldk-node-nodejs">
    <img src="https://img.shields.io/npm/v/ldk-node-nodejs" alt="Current npm package version." />
  </a>
    <a href="https://github.com/LtbLightning/ldk-node-nodejs/issues">
    <img src="https://img.shields.io/github/issues/LtbLightning/ldk-node-rn.svg" alt="Issues" />
  </a>
  <a href="https://github.com/LtbLightning/ldk-node-nodejs/stargazers">
    <img src="https://img.shields.io/github/stars/LtbLightning/ldk-node-rn.svg" alt="Stars" />
  </a>
  <a href="https://github.com/LtbLightning/ldk-node-nodejs/forks">
    <img src="https://img.shields.io/github/forks/LtbLightning/ldk-node-rn.svg?color=brightgreen" alt="Forks" />
  </a>
</p>

A NodeJs package for [LDK Node](https://github.com/lightningdevkit/ldk-node), a ready-to-go Lightning node library built using [LDK](https://lightningdevkit.org) and [BDK](https://bitcoindevkit.org).

LDK Node is a non-custodial Lightning node. Its central goal is to provide a small, simple, and straightforward interface that enables users to easily set up and run a Lightning node with an integrated on-chain wallet. While minimalism is at its core, LDK Node aims to be sufficiently modular and configurable to be useful for a variety of use cases.

The primary abstraction of the library is the Node, which can be retrieved by setting up and configuring a Builder to your liking and calling build(). Node can then be controlled via commands such as start, stop, connectOpenChannel, sendPayment, etc.:

This release covers the same API from LDK Node 0.1.0 Rust. It has support for sourcing chain data via an Esplora server, filesystem persistence, gossip sourcing via the Lightning peer-to-peer network, and configurable entropy sources for the integrated LDK and BDK-based wallets.

Please note: This release is considered experimental, and should not be run in production

## Installation

Using npm:

```bash
npm i ldk-node
```

Using yarn:

```bash
yarn add ldk-node
```

### Examples

### Build, Start & Sync the local node

```js
import {LogLevel, Network, Config, Builder, NetAddress, Node} from 'ldk-node';

// ....

// Your preferred `Esplora` url
const esploraUrl = https://blockstream.info/testnet/api;

let storageDirPath = `temp`
let logDirPath = `${storageDirPath}/logs`
let listeningAddress = new NetAddress('127.0.0.1', 2000)

// configuration options for the node
const config = new Config(storageDirPath, logDirPath, Network.Regtest, listeningAddress, 12, 12, 12, 12, 12, LogLevel.Debug)

const builder = new Builder().fromConfig(config);
builder.setEsploraServer(esploraUrl);

// Build node
const node: Node = builder.build();

// Starting the node
node.start();

// Syncing the node
node.syncWallets();

// get total onChain balanace
node.totalOnchainBalanceSats()

```

### References:

- Setting up a local Esplora instance for testing:
  https://bitcoin.stackexchange.com/questions/116937/how-do-i-setup-an-esplora-instance-for-local-testing/116938#116938
