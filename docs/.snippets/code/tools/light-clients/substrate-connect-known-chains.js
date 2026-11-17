import { createScClient, WellKnownChain } from '@substrate/connect';
import { chainSpec as assetHubSpec } from '@substrate/connect-known-chains/polkadot_asset_hub';

const client = createScClient();

// Add a well-known relay chain by name. If a light client extension is
// installed, it may already have this chain synced.
const polkadot = await client.addWellKnownChain(WellKnownChain.polkadot);

// Add a parachain using a chain spec from @substrate/connect-known-chains.
const assetHub = await polkadot.addChain(assetHubSpec);

assetHub.sendJsonRpc(
  JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'system_chain', params: [] })
);
console.log(JSON.parse(await assetHub.nextJsonRpcResponse()).result);

assetHub.remove();
polkadot.remove();
