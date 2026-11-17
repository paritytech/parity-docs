---
title: The resolc Compiler
description: Install and use resolc, the Revive compiler that turns Solidity into PolkaVM bytecode, with its CLI flags, standard JSON settings, and solc requirements.
short_description: Compile Solidity to PolkaVM bytecode with resolc.
categories: Smart Contracts, Tooling
---

# The resolc Compiler

## Introduction

[`resolc`](https://github.com/paritytech/revive) is the Solidity compiler for the PVM execution backend of `pallet-revive`. It compiles Solidity `v0.8` contracts to PolkaVM (PVM) bytecode, which runs on the [RISC-V](https://en.wikipedia.org/wiki/RISC-V)-based [PolkaVM](https://github.com/paritytech/polkavm) virtual machine.

`revive` is the name of the overall compiler project, which includes the Yul parser, the code generation library, and other components. `resolc` is the compiler driver executable that combines those components into a single command-line tool. This page covers how to install `resolc`, compile contracts with it, and configure its PVM-specific settings.

Contracts that target the REVM backend do not need `resolc`; they use standard `solc` output. Contracts compiled with `resolc` run on PVM, which behaves differently from the EVM in several areas, such as gas, memory, and contract creation. Read [EVM vs PVM](/smart-contracts/for-eth-devs/evm-vs-pvm/) before you deploy a contract compiled with `resolc`.

## How resolc Compiles Contracts

`resolc` does not parse Solidity itself. It runs the Ethereum reference compiler, `solc`, to produce the Yul intermediate representation (IR) of each contract, and then lowers that Yul to PVM code through a custom build of [LLVM](https://llvm.org/) with a RISC-V target. The intermediate artifacts that `resolc` can write with `--debug-output-dir` reflect this pipeline: the Yul IR, the unoptimized and optimized LLVM IR, the ELF object, and the PVM assembly.

Because `resolc` depends on `solc` at compile time, you need both compilers. The [`resolc` npm package](#install-the-npm-package) bundles `solc`; the [standalone binary](#install-a-binary-release) expects you to [install `solc`](#install-solc) separately.

## Install resolc

You can install `resolc` as a standalone binary, as an npm package, or by building it from source.

### Install a Binary Release

`resolc` is distributed as a standalone binary whose only external dependency is `solc`. Each [GitHub release](https://github.com/paritytech/revive/releases) includes binaries for the following platforms:

- **Linux**: `resolc-x86_64-unknown-linux-musl`.
- **macOS (universal)**: `resolc-universal-apple-darwin`.
- **Windows**: `resolc-x86_64-pc-windows-msvc.exe`.
- **Wasm (Emscripten)**: `resolc.js`, `resolc.wasm`, and `resolc_web.js`.

To install version `{{ dependencies.javascript_packages.resolc.version }}` on Linux or macOS:

=== "Linux"

    ```bash
    curl -L -o resolc https://github.com/paritytech/revive/releases/download/v{{ dependencies.javascript_packages.resolc.version }}/resolc-x86_64-unknown-linux-musl
    chmod +x resolc
    ```

=== "macOS"

    ```bash
    curl -L -o resolc https://github.com/paritytech/revive/releases/download/v{{ dependencies.javascript_packages.resolc.version }}/resolc-universal-apple-darwin
    xattr -rc resolc
    chmod +x resolc
    ```

    On macOS, `xattr -rc` clears the downloaded attribute so the system allows the binary to run.

Move the binary to a directory on your `PATH`, then check the installed version:

```bash
resolc --version
```

Nightly builds are listed on the [`resolc-bin` page](https://paritytech.github.io/resolc-bin/#nightly).

### Install the npm Package

The [`{{ dependencies.javascript_packages.resolc.name }}`](https://www.npmjs.com/package/@parity/resolc) package contains `resolc` compiled to WebAssembly together with the [`solc`](https://www.npmjs.com/package/solc) npm package, so it needs no separate `solc` installation. Install it in your project:

```bash
npm install --save-dev {{ dependencies.javascript_packages.resolc.name }}@{{ dependencies.javascript_packages.resolc.version }}
```

The package provides a `resolc` command for use with `npx`. You must pass `--bin`, `--abi`, or both:

```bash
npx resolc --bin --abi contracts/Storage.sol -o build
```

For each contract, the command writes the PVM bytecode to a `.polkavm` file and the ABI to a `.abi` file in the output directory. It accepts the following options:

| Option | Description |
|:------:|:-----------:|
| `--bin` | Write the PVM bytecode of each contract |
| `--abi` | Write the ABI of each contract |
| `-o`, `--output-dir` | Output directory for the generated files (default: the current directory) |
| `--base-path` | Root of the project source tree, used to resolve imports |
| `--include-path` | Extra source directories for resolving imports, such as installed packages |
| `-p`, `--pretty-json` | Pretty-print JSON output |
| `-v`, `--verbose` | Print more detailed console output |
| `--diff-stats` | Compare the size of the `resolc` output with `solc` output |

The package also exports a `compile` function for use from Node.js. It takes a map of source names to contents and returns the standard JSON output:

```ts
import { readFileSync } from 'fs';
import { compile } from '@parity/resolc';

const sources = {
  'contracts/Storage.sol': {
    content: readFileSync('contracts/Storage.sol', 'utf8'),
  },
};

const output = await compile(sources);
```

`compile` accepts an optional second argument with an `optimizer` object, which replaces the default `{ mode: 'z', enabled: true, runs: 200 }`, and a `bin` path. When you set `bin` to the path of a native `resolc` binary, `compile` runs that binary in standard JSON mode instead of the bundled WebAssembly build.

!!! note
    The npm package does not expose every option of the `resolc` binary. Use the binary when you need options such as `--heap-size`, `--stack-size`, or linker mode. For the package's source, see [`js/resolc`](https://github.com/paritytech/revive/tree/v{{ dependencies.javascript_packages.resolc.version }}/js/resolc) in the `revive` repository.

### Build from Source

Building `resolc` requires a stable [Rust](https://rustup.rs/) installation, a C++ toolchain, and a custom LLVM build with the RISC-V embedded target. You can download a prebuilt LLVM from the `revive` releases or build it yourself; the [`revive` README](https://github.com/paritytech/revive#building-from-source) covers both options. The following commands, run from a clone of the repository, build LLVM from source and then install `resolc`:

```bash
make install-llvm
export LLVM_SYS_231_PREFIX=${PWD}/target-llvm/gnu/target-final
make install-bin
resolc --version
```

### Install solc

The `resolc` binary calls `solc` during compilation. By default, it uses the `solc` executable found on your `PATH`; use `--solc` to point it at a different executable. To install `solc`, follow the [Solidity installation instructions](https://docs.soliditylang.org/en/latest/installing-solidity.html).

`resolc` {{ dependencies.javascript_packages.resolc.version }} supports `solc` versions `0.8.0` through `0.8.38`. To print the supported range for your installed version, run:

```bash
resolc --supported-solc-versions
```

The npm package declares the same range for its bundled `solc` dependency (`>=0.8.0 <=0.8.38`).

## Compile Contracts with the CLI

The `resolc` CLI is designed to stay close to the `solc` CLI. For example, the following command compiles a contract with the `-O3` optimization level and writes the PVM bytecode to the `build` directory:

```bash
resolc contracts/Storage.sol -O3 --bin --output-dir ./build
```

Pass `--overwrite` to replace files that already exist in the output directory. Run `resolc --help` for the complete list of options.

The following sections list the main flags, as defined in the [`resolc` CLI source](https://github.com/paritytech/revive/blob/v{{ dependencies.javascript_packages.resolc.version }}/crates/resolc/src/resolc/arguments.rs).

### Input and Output Flags

| Flag | Description |
|:----:|:-----------:|
| `<INPUTS>` | Input files and import remappings. An argument that contains `=` is treated as a remapping. Multiple files are allowed in Solidity mode |
| `-o`, `--output-dir` | Write one file per component and contract to this directory |
| `--overwrite` | Overwrite existing files in the output directory |
| `--bin` | Output the PVM bytecode of the contracts |
| `--asm` | Output the PVM assembly of the contracts |
| `--metadata` | Output the metadata of the compiled project |
| `--combined-json` | Output a single JSON document with the selected fields: `abi`, `hashes`, `metadata`, `devdoc`, `userdoc`, `storage-layout`, `ast`, `asm`, `bin`, `bin-runtime` |
| `--standard-json` | Use [standard JSON](#use-standard-json-input-and-output) input and output |
| `--yul` | Compile a single Yul file instead of Solidity |
| `--base-path`, `--include-path`, `--allow-paths` | Import resolution settings, passed to `solc` unchanged |

### Compiler Settings Flags

| Flag | Description | Default |
|:----:|:-----------:|:-------:|
| `-O`, `--optimization` | LLVM optimization level: `0`, `1`, `2`, `3`, `s`, or `z` | `z` |
| `--disable-solc-optimizer` | Disable the `solc` optimizer, which `resolc` otherwise always enables | Off |
| `--solc` | Path to the `solc` executable | `solc` on `PATH` |
| `--evm-version` | EVM target version to generate IR for | None |
| `--revert-strings` | Revert and require reason strings: `default`, `strip`, or `debug` | None |
| `--metadata-hash` | Metadata hash type: `none` or `keccak256` (`ipfs` is rejected) | `keccak256` |
| `--heap-size` | Size in bytes of the static buffer that emulates EVM heap memory | `131072` |
| `--stack-size` | Total contract stack size in bytes | `131072` |
| `--suppress-warnings` | Suppress warnings: `ecrecover`, `sendtransfer`, `extcodesize`, `txorigin`, `blocktimestamp`, `blocknumber`, `blockhash` | None |
| `--newyork` | Route Yul lowering through the experimental newyork IR pipeline | Off |

### Linking, Debugging, and Information Flags

| Flag | Description |
|:----:|:-----------:|
| `-l`, `--libraries` | Addresses of deployable libraries, in the form `<libraryName>=<address>` |
| `--link` | Linker mode: link unlinked ELF objects into deployable PVM blobs |
| `-g` | Generate source-based debug information |
| `--debug-output-dir` | Write all intermediate compiler artifacts to this directory |
| `--llvm-arg` | Pass an argument to LLVM |
| `--version` | Print the compiler version |
| `--supported-solc-versions` | Print the supported `solc` versions |

Only one of the Yul, combined JSON, standard JSON, and linker modes can be active at a time.

## Configure Optimization

`resolc` applies two optimizers:

- **The `solc` optimizer**: `resolc` always enables it, so that `solc` can inline internal library calls. Disable it with `--disable-solc-optimizer` only when you need to, for example, if your project uses the `MSIZE` instruction. Disabling it can prevent libraries from being inlined.
- **The LLVM optimizer**: Set its level with `-O`. The levels are `0` (no optimization), `1` (basic), `2` (advanced), and `3` (aggressive) optimization for execution time, and `s` and `z` to optimize for code size and aggressively for code size. The default is `z`.

The performance and size of compiled contracts vary widely between optimization levels. Use `-O3` when execution speed matters most and `-Oz` when code size matters most.

The `--newyork` flag is independent of the optimization level. It routes Yul lowering through the [newyork IR pipeline](https://paritytech.github.io/revive/developer_guide/newyork_optimizer.html), an experimental pipeline that aims to reduce code size.

## Configure Memory

PVM contracts cannot grow their memory dynamically the way EVM contracts do. `resolc` emulates EVM heap memory with a fixed-size buffer and gives each contract a fixed amount of stack memory, so a contract has a finite amount of memory at constant gas cost.

Set these sizes with `--heap-size` and `--stack-size`, both 131072 bytes by default. Keep them as small as your contract allows: larger sizes increase the startup cost of every call, and both count toward the total memory a contract can use, which includes its code size.

!!! warning
    A contract that uses more heap or stack memory than configured compiles without errors but reverts at runtime when it exceeds the limit. Test contracts with realistic inputs after changing these values.

For the memory limits that `pallet-revive` enforces, see [Memory Management](/smart-contracts/for-eth-devs/evm-vs-pvm/#memory-management).

## Use Standard JSON Input and Output

In standard JSON mode, `resolc` reads a JSON input document from standard input and writes the JSON output to standard output. Build tools such as the Hardhat plugin use this mode:

```bash
resolc --standard-json < input.json > output.json
```

The format follows the [`solc` standard JSON interface](https://docs.soliditylang.org/en/latest/using-the-compiler.html#compiler-input-and-output-json-description), with additional PVM settings. The following input compiles one source file, passed inline as `content`, requests the ABI and bytecode for all contracts, and sets the PVM memory sizes:

```json title="input.json"
{
  "language": "Solidity",
  "sources": {
    "contracts/Storage.sol": {
      "content": "INSERT_SOLIDITY_SOURCE"
    }
  },
  "settings": {
    "optimizer": {
      "enabled": true,
      "mode": "z",
      "runs": 200
    },
    "polkavm": {
      "memoryConfig": {
        "heapSize": 131072,
        "stackSize": 131072
      }
    },
    "outputSelection": {
      "*": {
        "*": ["abi", "evm.bytecode"]
      }
    }
  }
}
```

The PVM-specific input settings are:

- **`settings.optimizer.mode`**: The LLVM optimization level as a single character, equivalent to `-O`.
- **`settings.polkavm.memoryConfig.heapSize`**: The heap size in bytes, equivalent to `--heap-size`.
- **`settings.polkavm.memoryConfig.stackSize`**: The stack size in bytes, equivalent to `--stack-size`.
- **`settings.polkavm.debugInformation`**: Set to `true` to generate debug information, equivalent to `-g`.
- **`settings.polkavm.newyork`**: Set to `true` to enable the experimental newyork pipeline. The `--newyork` flag is rejected in standard JSON mode, so use this setting instead.
- **`settings.llvmArguments`**: Arbitrary LLVM arguments, mainly for development and debugging.

In `settings.outputSelection`, `resolc` accepts the `*` wildcard for file names and contract names, but contract-level keys can only be `*`. When you request code generation outputs such as `evm.bytecode`, `resolc` internally adds the `ast`, `metadata`, `irOptimized`, and `evm.methodIdentifiers` selectors it needs, but includes them in the output only if you request them.

In the output, `evm.bytecode.object` holds the PVM bytecode in hexadecimal form. The top-level `resolc_pipeline` field reports which pipeline compiled the contracts: `yul` for the standard pipeline or `newyork`.

To detect libraries that must be deployed separately without compiling contracts, combine `--standard-json` with `--detect-missing-libraries`.

For the full description of the standard JSON differences, see the [Standard JSON interface](https://paritytech.github.io/revive/user_guide/std_json.html) chapter of the `revive` book.

## Link Libraries

`solc` inlines most library calls, and `resolc` always enables the `solc` optimizer that performs this inlining. If you pass library addresses with `--libraries` (or the equivalent standard JSON setting), `resolc` links externally callable libraries at compile time.

If a library address is unknown at compile time, `resolc` emits the affected contracts as unlinked ELF objects that you cannot deploy yet. To make them deployable, run `resolc --link` with `--libraries`. Unlike `solc`, the linker must resolve ELF symbols, so you must pass the unlinked files in the same directory structure as their source files. Deploy-time linking is discouraged because it changes bytecode hashes after compilation.

For details, see the [CLI usage](https://paritytech.github.io/revive/user_guide/cli.html#deploy-time-linking) chapter of the `revive` book.

## Use resolc with Development Tools

- **Hardhat**: The [Hardhat Polkadot](/smart-contracts/dev-environments/hardhat-polkadot/) plugin compiles contracts with `resolc`.
- **Foundry**: The [Parity Foundry fork](https://github.com/paritytech/foundry-polkadot) integrates `resolc`.
- **Compiler Explorer**: `resolc` is available on [godbolt.org](https://godbolt.org/) for Solidity and Yul input.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge learn">Learn</span> __EVM vs PVM__

    ---

    Review the behavioral differences between contracts compiled with `resolc` for PVM and contracts running on the EVM.

    [:octicons-arrow-right-24: Reference](/smart-contracts/for-eth-devs/evm-vs-pvm/)

-   <span class="badge guide">Guide</span> __Hardhat Polkadot__

    ---

    Compile, test, and deploy PVM contracts with Hardhat and `resolc`.

    [:octicons-arrow-right-24: Get Started](/smart-contracts/dev-environments/hardhat-polkadot/)

</div>
