---
title: Utilities
description: Interactive encoding, hashing, and address utilities for Polkadot SDK development. Convert values to hex, hash inputs with Blake2 and XXHash, re-encode addresses, and derive pallet-revive ERC-20 precompile addresses.
categories: Tooling
extra_javascript:
  - https://cdn.jsdelivr.net/npm/@polkadot/util@14.0.1/bundle-polkadot-util.js
  - https://cdn.jsdelivr.net/npm/@polkadot/util-crypto@14.0.1/bundle-polkadot-util-crypto.js
  - https://cdn.jsdelivr.net/npm/@polkadot/keyring@14.0.1/bundle-polkadot-keyring.js
  - /js/account-converter.js
  - /js/erc20-asset-converter.js
  - /js/polkadot-utilities.js
extra_css:
  - /assets/stylesheets/account-converter.css
  - /assets/stylesheets/erc20-asset-converter.css
  - /assets/stylesheets/polkadot-utilities.css
hide:
  - navigation
---

# Utilities

This page provides interactive utilities for common Polkadot SDK development tasks. Tools are powered by the [@polkadot/util](https://github.com/polkadot-js/common/tree/master/packages/util), [@polkadot/util-crypto](https://github.com/polkadot-js/common/tree/master/packages/util-crypto), and [@polkadot/keyring](https://github.com/polkadot-js/common/tree/master/packages/keyring) libraries and run entirely in your browser.

## Encoding & Decoding

Convert between strings, numbers, byte arrays, and their hexadecimal representations. Bidirectional tools update both fields in real time as you type.

<div id="encoding-root"></div>

## Hashing

Hash arbitrary inputs using Blake2 and XXHash, the two hashing algorithms used throughout the Polkadot SDK for storage key derivation and other primitives. The **concat** variants append the original input to the hash output, matching the `Blake2_128Concat` and `Twox64Concat` storage hashers.

<div id="hashing-root"></div>

## Address Utilities

Derive, convert, and re-encode addresses for Polkadot SDK-based chains. These tools cover standard SS58 addresses, Ethereum-compatible addresses, pallet module accounts, sovereign accounts for parachains, and derived sub-accounts.

!!! warning
    Never enter seed phrases or private key URIs for accounts that hold real funds. Use only test accounts or development keys (for example, `//Alice`) with the Seed → Address tool.

<div id="address-root"></div>

Convert addresses between Ethereum (EVM) and Polkadot (SS58) formats. Use the network selector to target a specific chain's SS58 prefix. When converting SS58 → ETH, check the box to confirm the account was originally mapped from an Ethereum key.

<div id="account-converter-root"></div>

## Asset ID to ERC-20 Precompile Address

Derive the ERC-20 precompile address for an asset ID. On chains running `pallet-revive`, a runtime can expose each Assets pallet instance through an [ERC-20 precompile](/smart-contracts/precompiles/erc20/), and contracts use the derived address to call that asset through the standard ERC-20 interface.

The address suffix is runtime configuration. The tool uses the suffixes from the Asset Hub runtimes in the Polkadot SDK (`0x0120` for trust-backed assets, `0x0220` for foreign assets, and `0x0320` for pool assets; see the [Asset Hub Westend runtime](https://github.com/paritytech/polkadot-sdk/blob/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/runtimes/assets/asset-hub-westend/src/lib.rs)). If your runtime uses different suffixes, adjust the result accordingly.

<div id="erc20-converter-root"></div>
