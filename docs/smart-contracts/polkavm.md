---
title: PolkaVM
description: Learn what PolkaVM is, how pallet-revive executes PolkaVM program blobs alongside EVM bytecode, and which toolchains produce PVM contracts from Solidity and Rust.
short_description: Understand PolkaVM and the toolchains that target it.
categories: Smart Contracts, Basics
---

# PolkaVM

## Introduction

[PolkaVM](https://github.com/paritytech/polkavm/tree/v{{ dependencies.crates.polkavm.version }}) is a general-purpose, user-level virtual machine based on the [RISC-V](https://riscv.org/) instruction set. `pallet-revive` embeds PolkaVM as its PVM execution backend, next to the EVM backend that runs standard Solidity bytecode.

This page explains how PolkaVM programs are built, how `pallet-revive` recognizes and executes them, and which toolchains produce them. For the behavioral differences that Solidity developers see on PVM, such as memory limits, gas, and contract creation, see [EVM vs PVM](/smart-contracts/for-eth-devs/evm-vs-pvm.md). For the full smart contract stack, see the [Smart Contracts Overview](/smart-contracts/overview.md).

## What PolkaVM Is

PolkaVM executes programs compiled for a RISC-V target instead of a stack machine such as the EVM. Its [design goals](https://github.com/paritytech/polkavm/tree/v{{ dependencies.crates.polkavm.version }}#design-goals) include the following:

- Sandboxed execution that keeps guest code away from the host system.
- Fully deterministic execution, so the same code and inputs always produce the same output.
- Fast, single-pass compilation, so loading new code is near-instantaneous.
- Small program binaries and a low memory footprint per instance.
- Cheap, deterministic gas metering.

!!! note
    The PolkaVM README lists these as design goals and states that not all of them are met yet. Refer to the [PolkaVM repository](https://github.com/paritytech/polkavm/tree/v{{ dependencies.crates.polkavm.version }}) for the project's status.

### From Source Code to a Program Blob

A compiler does not emit PolkaVM code directly. Instead, a PVM toolchain produces a RISC-V ELF file for the `riscv64emac-unknown-none-polkavm` target and then links it into a PolkaVM program blob with `polkavm-linker`. The linker rewrites the ELF into PolkaVM's own compact format, which is the artifact you deploy, conventionally stored in a `.polkavm` file.

Every program blob starts with the four magic bytes `PVM\0`, followed by a version byte and the blob length. PolkaVM programs expose named entry points, called exports, that the host can call, and they reach the outside world only through host functions that the embedding environment provides.

The [`polkatool`](https://crates.io/crates/polkatool) command-line tool from the PolkaVM repository works with program blobs. Its subcommands include `link` (ELF to `.polkavm`), `disassemble`, `assemble`, `stats`, and `get-target-json-path`.

## How pallet-revive Executes Contracts

`pallet-revive` decides which backend runs a contract by inspecting the code you upload:

- **PVM**: Code that starts with the PolkaVM magic bytes is treated as a program blob and executed by PolkaVM.
- **EVM**: Any other code is treated as EVM bytecode and executed by the EVM backend, but only if the runtime sets the `AllowEVMBytecode` configuration item of `pallet-revive` to `true`. Otherwise, the upload is rejected.

Both kinds of contracts share the same account model, storage, Ethereum JSON-RPC interface, and [precompiles](/smart-contracts/precompiles/index.md), and they can call each other.

### The Contract Interface

A PVM contract is a program blob with two exports:

- **`deploy`**: The constructor, called once when the contract is instantiated.
- **`call`**: The entry point for every later call to the contract.

Neither export takes arguments or returns a value. A contract reads its input with host functions such as `call_data_size` and `call_data_copy`, and ends execution with `return_value`, which carries the output data and a flag that marks a revert. Other host functions give access to storage, events, balances, block information, and calls to other contracts. The [`pallet-revive-uapi`](https://docs.rs/pallet-revive-uapi/{{ dependencies.crates.pallet_revive_uapi.version }}/pallet_revive_uapi/) crate defines this interface for contract code; its `HostFn` trait documents every host function. `pallet-revive` passes call data to the contract unchanged, so a PVM contract that decodes its input and encodes its output with the Solidity ABI can be called by Ethereum tools like any other contract.

### Deployment

`pallet-revive` stores PVM code separately from the contracts that use it. You can upload code on its own with the `upload_code` dispatchable and instantiate contracts from its code hash later, or upload and instantiate in one step. Only PVM code can be instantiated from a code hash; EVM contracts are always deployed from their init code. See [Contract Deployment](/smart-contracts/for-eth-devs/contract-deployment.md) for how this affects factory contracts.

When you deploy a PVM contract with an Ethereum transaction (a transaction without a `to` address), the transaction data is the program blob followed by the constructor input. `pallet-revive` reads the blob length from the blob header, splits the data at that point, uploads the code, and passes the rest to the `deploy` export.

## Toolchains That Produce PVM Bytecode

You can produce PVM contracts from Solidity or from Rust:

- **Solidity with `resolc`**: The [`resolc` compiler](/smart-contracts/resolc.md) compiles Solidity, through the Yul IR that `solc` emits and a custom LLVM build, to PolkaVM program blobs. Use it when you have Solidity contracts that should run on PVM.
- **Rust with `cargo-pvm-contract`**: [`cargo-pvm-contract`](https://crates.io/crates/cargo-pvm-contract/{{ dependencies.crates.cargo_pvm_contract.version }}) is a Cargo subcommand that scaffolds Rust contract projects and builds them into `.polkavm` files. The generated contracts are `no_std` Rust programs that call host functions through `pallet-revive-uapi` and mark their exports with `polkavm-derive`. This is a low-level way to write contracts, well suited to small, performance-sensitive code that Solidity contracts call. See [Write Rust Contracts](/smart-contracts/rust-contracts.md).
- **ink!**: [ink!](https://github.com/use-ink/ink) is a third-party Rust smart contract language whose version 6 targets `pallet-revive`; its repository states that it has not been actively maintained since January 2026.

The [contract fixtures](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/fixtures/contracts) that `pallet-revive` uses for its tests are written in Rust against `pallet-revive-uapi` and show how to use most host functions.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Write Rust Contracts__

    ---

    Scaffold, build, and deploy a Rust contract for `pallet-revive` with `cargo-pvm-contract`.

    [:octicons-arrow-right-24: Get Started](/smart-contracts/rust-contracts.md)

-   <span class="badge guide">Guide</span> __The resolc Compiler__

    ---

    Compile Solidity contracts to PolkaVM bytecode.

    [:octicons-arrow-right-24: Get Started](/smart-contracts/resolc.md)

-   <span class="badge learn">Learn</span> __EVM vs PVM__

    ---

    Review the behavioral differences between contracts on PVM and contracts on the EVM.

    [:octicons-arrow-right-24: Reference](/smart-contracts/for-eth-devs/evm-vs-pvm.md)

</div>
