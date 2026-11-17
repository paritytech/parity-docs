---
title: Write Rust Contracts
description: Scaffold, build, and deploy a Rust smart contract for pallet-revive with cargo-pvm-contract and pallet-revive-uapi, then call it on a local development node.
short_description: Build and deploy Rust contracts for the PVM backend.
categories: Smart Contracts, Tooling
---

# Write Rust Contracts

## Introduction

You can write contracts for the PVM backend of `pallet-revive` directly in Rust. A Rust contract is a `no_std` program that exports a `deploy` and a `call` function, talks to the chain through the host functions in [`pallet-revive-uapi`](https://docs.rs/pallet-revive-uapi/{{ dependencies.crates.pallet_revive_uapi.version }}/pallet_revive_uapi/), and is linked into a PolkaVM program blob. For background on program blobs and how `pallet-revive` runs them, see [PolkaVM](/smart-contracts/polkavm.md).

[`cargo-pvm-contract`](https://crates.io/crates/cargo-pvm-contract/{{ dependencies.crates.cargo_pvm_contract.version }}) is a Cargo subcommand that creates Rust contract projects and configures them so that `cargo build` produces a deployable `.polkavm` file. It can generate the function selectors and ABI decoding code for a contract from a Solidity interface, so Ethereum tools and Solidity contracts can call the result.

This is a low-level way to write contracts: you handle call data, storage keys, and return values yourself. It suits small, performance-sensitive code, such as a library that Solidity contracts call. In this guide, you create a contract from the Fibonacci example, build it, deploy it to a local development node, and call it.

!!! warning "Provisional"
    `cargo-pvm-contract` {{ dependencies.crates.cargo_pvm_contract.version }} generates projects that depend on `pallet-revive-uapi` 0.10 and link with PolkaVM 0.31, while Polkadot SDK `{{dependencies.repositories.polkadot_sdk.version}}` ships `pallet-revive-uapi` {{ dependencies.crates.pallet_revive_uapi.version }} and PolkaVM {{ dependencies.crates.polkavm.version }}. The development branch of `cargo-pvm-contract` moves to the newer versions but has no release yet. Confirm that your contract deploys on the runtime you target.

## Prerequisites

Before you start, make sure you have:

- [Rust installed with `rustup`](/polkadot-sdk/install-polkadot-sdk.md). Generated projects select the nightly toolchain through a `rust-toolchain.toml` file and build the Rust core library from source, so install the nightly toolchain with the `rust-src` component:

    ```bash
    rustup toolchain install nightly --component rust-src
    ```

- The Solidity compiler, `solc`, on your `PATH`. `cargo-pvm-contract` runs it to read the ABI of the Solidity interface when it creates a project from an example or from a `.sol` file. A blank project does not need it.
- A local development node and the `eth-rpc` adapter, built as described in [Local Development Node](/smart-contracts/dev-environments/local-dev-node.md). You need them to deploy and call the contract.

## Install cargo-pvm-contract

Install the subcommand from crates.io:

```bash
cargo install --force --locked cargo-pvm-contract@{{ dependencies.crates.cargo_pvm_contract.version }}
```

## Create a Contract Project

Run the subcommand without arguments to choose the project type, memory model, and name interactively:

```bash
cargo pvm-contract
```

You can also pass every choice as a flag. The following command creates a project named `fibonacci` from the Fibonacci example without a memory allocator:

```bash
cargo pvm-contract --init-type example --example Fibonacci --memory-model no-alloc --name fibonacci
```

The subcommand accepts the following flags:

| Flag | Values | Description |
|:----:|:------:|:-----------:|
| `--init-type` | `solidity-file`, `example`, `blank` | Create the project from your own Solidity interface, from a bundled example, or as an empty contract |
| `--example` | `Fibonacci`, `MyToken` | The bundled example to start from, with `--init-type example` |
| `--sol-file` | Path | The Solidity interface file to generate the contract from, with `--init-type solidity-file` |
| `--memory-model` | `alloc-with-alloy`, `no-alloc` | Whether the contract uses a heap allocator and `alloy-core` for ABI encoding |
| `--name` | Name | The contract name, also used as the project directory name |

The two memory models trade convenience for binary size:

- **`no-alloc`**: The generated code declares each function selector, event signature, and error selector as a constant and decodes arguments from a fixed-size call data buffer. Contracts are smaller, but you encode and decode values by hand.
- **`alloc-with-alloy`**: The generated code uses the `sol!` macro from [`alloy-core`](https://crates.io/crates/alloy-core) to create typed call structures from the Solidity file, and a `picoalloc` allocator over a 1 KiB static buffer to provide `alloc`. The API is easier to use, and the binary is larger.

The command creates a `fibonacci` directory with the following files:

- **`Cargo.toml`**: The package manifest. It defines a binary target named after the contract and depends on `pallet-revive-uapi` and `polkavm-derive`, with `cargo-pvm-contract-builder` as a build dependency. Its release profile optimizes for size.
- **`build.rs`**: A build script that calls `cargo_pvm_contract_builder::PvmBuilder::new().build()`.
- **`.cargo/config.toml`**: Sets the PolkaVM target as the default build target and enables building `core` and `alloc` from source.
- **`riscv64emac-unknown-none-polkavm.json`**: The target specification for PolkaVM.
- **`rust-toolchain.toml`**: Selects the nightly toolchain.
- **`Fibonacci.sol`**: The Solidity interface that the contract implements.
- **`src/fibonacci.rs`**: The contract source.

## Review the Contract Code

The Solidity interface declares one function:

```solidity title="Fibonacci.sol"
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.0;

interface Fibonacci {
    function fibonacci(uint32) external pure returns (uint32);
}
```

The generated contract implements it. Some generated comments are omitted here:

```rust title="src/fibonacci.rs"
#![no_main]
#![no_std]

use pallet_revive_uapi::{HostFn, HostFnImpl as api, ReturnFlags};

// Function selectors
const FIBONACCI_SELECTOR: [u8; 4] = [0xe4, 0x44, 0xa7, 0x09]; // fibonacci(uint32)

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Safety: The unimp instruction is guaranteed to trap
    unsafe {
        core::arch::asm!("unimp");
        core::hint::unreachable_unchecked();
    }
}

/// This is the constructor which is called once per contract.
#[polkavm_derive::polkavm_export]
pub extern "C" fn deploy() {}

/// This is the regular entry point when the contract is called.
#[polkavm_derive::polkavm_export]
pub extern "C" fn call() {
    let call_data_len = api::call_data_size() as usize;

    // Fixed buffer for call data
    let mut call_data = [0u8; 256];
    if call_data_len > call_data.len() {
        panic!("Call data too large");
    }

    api::call_data_copy(&mut call_data[..call_data_len], 0);

    if call_data_len < 4 {
        panic!("Call data too short");
    }

    let selector: [u8; 4] = call_data[0..4].try_into().unwrap();

    match selector {
        FIBONACCI_SELECTOR => {
            if call_data_len < 36 {
                panic!("Invalid fibonacci call data");
            }

            let mut input = [0u8; 4];
            api::call_data_copy(&mut input, 32);

            let n = u32::from_be_bytes(input);
            let result = _fibonacci(n);
            let output = result.to_be_bytes();

            let mut response = [0u8; 32];
            response[28..].copy_from_slice(&output);
            api::return_value(ReturnFlags::empty(), &response);
        }

        _ => panic!("Unknown function selector"),
    }
}

fn _fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        _fibonacci(n - 1) + _fibonacci(n - 2)
    }
}
```

The contract has the following parts:

- **Crate attributes**: `#![no_std]` and `#![no_main]` remove the standard library and the usual `main` entry point, because the contract runs inside PolkaVM without an operating system.
- **Panic handler**: A `no_std` program must define one. This handler executes the `unimp` instruction, which traps and makes the call fail.
- **Exports**: `#[polkavm_derive::polkavm_export]` exports `deploy` and `call` from the program blob. `pallet-revive` calls `deploy` once when the contract is instantiated and `call` for every later call. The Fibonacci contract has no constructor logic, so `deploy` is empty.
- **Call data handling**: `call` reads the call data with `call_data_size` and `call_data_copy`, takes the first four bytes as the Solidity function selector, and reads the `uint32` argument from the last four bytes of the first 32-byte ABI word.
- **Return value**: `return_value` ends execution and returns the result, ABI-encoded as a 32-byte word. Passing `ReturnFlags::REVERT` instead of `ReturnFlags::empty()` reverts the call and its state changes.

When you create a project from your own Solidity interface, `cargo-pvm-contract` generates a branch with a `todo!()` placeholder for each function, which you replace with your logic. The [`HostFn`](https://docs.rs/pallet-revive-uapi/{{ dependencies.crates.pallet_revive_uapi.version }}/pallet_revive_uapi/trait.HostFn.html) trait documents the available host functions, such as `get_storage` and `set_storage` for contract storage, `deposit_event` for events, `caller` and `value_transferred` for call context, and `call` and `instantiate` for interacting with other contracts. The Rust [contract fixtures](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/fixtures/contracts) in the Polkadot SDK use most of these functions, for example [`storage.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/fixtures/contracts/storage.rs).

## Build the Contract

From the project directory, build the contract in release mode:

```bash
cd fibonacci
cargo build --release
```

The build script runs a second `cargo build` for the PolkaVM target, which compiles `core` and `alloc` from source, and then links the resulting ELF file into a PolkaVM program blob with `polkavm-linker`. The blob is written to `target/<bin>.<profile>.polkavm`, which for this project is `target/fibonacci.release.polkavm`. A plain `cargo build` writes `target/fibonacci.debug.polkavm` instead.

The `.polkavm` file is the complete contract code. You deploy it without further processing.

## Deploy the Contract to a Local Node

Deploy the contract with an Ethereum transaction that has no `to` address and carries the program blob as its data. `pallet-revive` recognizes the blob by its magic bytes and stores it as PVM code. If the constructor takes arguments, append their ABI encoding to the blob; the Fibonacci contract takes none.

This example uses the `eth_sendTransaction` method of the [`eth-rpc` adapter](/smart-contracts/eth-rpc-adapter.md). In development mode, the adapter signs transactions for five development accounts, which the Revive dev node funds at genesis. The example sends the transaction from the Alith development account, `0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac`.

1. Start the Revive dev node and the adapter in development mode, as described in [Run the Local Node](/smart-contracts/dev-environments/local-dev-node.md#run-the-local-node):

    ```bash
    ./target/release/revive-dev-node --dev
    ```

    ```bash
    ./target/release/eth-rpc --dev
    ```

2. In the contract project directory, encode the program blob as hexadecimal and send the deployment transaction:

    ```bash
    CODE=0x$(xxd -p target/fibonacci.release.polkavm | tr -d '\n')

    curl -s -X POST {{ networks.local.eth_rpc_url }} \
      -H "Content-Type: application/json" \
      -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_sendTransaction\",\"params\":[{\"from\":\"0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac\",\"data\":\"$CODE\"}]}"
    ```

    The adapter estimates the gas, signs the transaction, and returns the transaction hash in the `result` field.

3. Get the transaction receipt to find the address of the new contract:

    ```bash
    curl -s -X POST {{ networks.local.eth_rpc_url }} \
      -H "Content-Type: application/json" \
      -d '{"jsonrpc":"2.0","id":1,"method":"eth_getTransactionReceipt","params":["INSERT_TRANSACTION_HASH"]}'
    ```

    The `contractAddress` field of the receipt holds the contract address. A `status` of `0x1` means the deployment succeeded.

The development accounts exist only in the adapter's development mode. To deploy outside development mode, sign the transaction with your own key and submit it with `eth_sendRawTransaction`, for example from an Ethereum library or tool. Any tool that can send a contract creation transaction with arbitrary data can deploy the `.polkavm` file. See [JSON-RPC APIs](/smart-contracts/for-eth-devs/json-rpc-apis.md) for the methods the adapter supports.

## Call the Contract

Call `fibonacci(10)` with `eth_call`. The call data is the selector `0xe444a709` followed by the argument `10` encoded as a 32-byte word:

```bash
curl -s -X POST {{ networks.local.eth_rpc_url }} \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_call","params":[{"to":"INSERT_CONTRACT_ADDRESS","data":"0xe444a709000000000000000000000000000000000000000000000000000000000000000a"},"latest"]}'
```

The `result` field holds the ABI-encoded return value. The 10th Fibonacci number is 55, so the result ends in `37`, its hexadecimal value:

```text
0x0000000000000000000000000000000000000000000000000000000000000037
```

Because the contract uses the Solidity ABI, Solidity contracts can call it through the `Fibonacci` interface, and Ethereum libraries can call it with the interface's ABI.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge learn">Learn</span> __PolkaVM__

    ---

    Learn how `pallet-revive` recognizes and executes PolkaVM program blobs.

    [:octicons-arrow-right-24: Reference](/smart-contracts/polkavm.md)

-   <span class="badge learn">Learn</span> __Contract Deployment__

    ---

    Understand the code upload and instantiation model of the PVM backend.

    [:octicons-arrow-right-24: Reference](/smart-contracts/for-eth-devs/contract-deployment.md)

-   <span class="badge guide">Guide</span> __Add pallet-revive to Your Runtime__

    ---

    Run your contracts on your own chain by adding `pallet-revive` to its runtime.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/customize-runtime/add-pallet-revive.md)

</div>
