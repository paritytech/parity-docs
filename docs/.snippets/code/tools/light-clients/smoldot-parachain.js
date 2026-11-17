import { readFileSync } from 'node:fs';
import * as smoldot from 'smoldot';

const relayChainSpec = readFileSync('./relay-chain-spec.json', 'utf8');
const parachainSpec = readFileSync('./parachain-spec.json', 'utf8');

const client = smoldot.start();

// Add the relay chain first. Disable its JSON-RPC system if you only
// query the parachain.
const relayChain = await client.addChain({
  chainSpec: relayChainSpec,
  disableJsonRpc: true,
});

// Add the parachain. smoldot matches the parachain spec's relay chain ID
// against the chains listed in potentialRelayChains.
const parachain = await client.addChain({
  chainSpec: parachainSpec,
  potentialRelayChains: [relayChain],
});

parachain.sendJsonRpc(
  JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'system_chain', params: [] })
);
const response = JSON.parse(await parachain.nextJsonRpcResponse());
console.log(`Connected to parachain: ${response.result}`);

await client.terminate();
