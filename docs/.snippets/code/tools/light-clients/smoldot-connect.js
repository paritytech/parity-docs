import { readFileSync } from 'node:fs';
import * as smoldot from 'smoldot';

// Raw chain spec of the chain to connect to, with reachable bootnodes.
const chainSpec = readFileSync('./relay-chain-spec.json', 'utf8');

// Start the client. One client can connect to several chains.
const client = smoldot.start({ maxLogLevel: 3 });

// Add the chain. The promise resolves once the chain spec is parsed,
// before any network connection is opened.
const chain = await client.addChain({ chainSpec });

// Send a JSON-RPC request and wait for its response.
chain.sendJsonRpc(
  JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'system_chain', params: [] })
);
const response = JSON.parse(await chain.nextJsonRpcResponse());
console.log(`Connected to: ${response.result}`);

// Subscribe to finalized block headers. Notifications arrive through
// the same nextJsonRpcResponse() queue as responses.
chain.sendJsonRpc(
  JSON.stringify({
    jsonrpc: '2.0',
    id: 2,
    method: 'chain_subscribeFinalizedHeads',
    params: [],
  })
);

let received = 0;
while (received < 3) {
  const message = JSON.parse(await chain.nextJsonRpcResponse());
  if (message.method === 'chain_finalizedHead') {
    const header = message.params.result;
    console.log(`Finalized block #${parseInt(header.number, 16)}`);
    received += 1;
  }
}

// Remove all chains and shut down the client.
await client.terminate();
