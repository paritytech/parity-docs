import { createScClient } from '@substrate/connect';

// Load raw chain specs served next to the web page.
const relayChainSpec = await (await fetch('/relay-chain-spec.json')).text();
const parachainSpec = await (await fetch('/parachain-spec.json')).text();

// Use the in-page light client even if a light client extension is installed.
const client = createScClient({ forceEmbeddedNode: true });

// Add the relay chain, then add the parachain on top of it.
const relayChain = await client.addChain(relayChainSpec);
const parachain = await relayChain.addChain(parachainSpec);

parachain.sendJsonRpc(
  JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'system_chain', params: [] })
);
const response = JSON.parse(await parachain.nextJsonRpcResponse());
console.log(`Connected to parachain: ${response.result}`);

// Disconnect. smoldot keeps a relay chain alive while a parachain uses it.
parachain.remove();
relayChain.remove();
