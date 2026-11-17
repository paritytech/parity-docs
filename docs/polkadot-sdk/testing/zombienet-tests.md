---
title: Write Network Tests with Zombienet
description: Write end-to-end tests for Polkadot SDK networks with the Zombienet test DSL or the zombienet-sdk Rust crate, and run them locally or in CI.
categories: Polkadot SDK, Tooling
---

# Write Network Tests with Zombienet

## Introduction

[Zombienet](/tools/zombienet.md) spawns ephemeral networks of real nodes: relay chain validators, parachain collators, and the processes around them. On top of spawning, it lets you write tests that assert on the running network, such as whether nodes are up, whether a parachain is registered and producing blocks, what nodes report in their Prometheus metrics and logs, and what custom scripts return.

You can write these tests in two ways:

- **Zombienet test DSL**: A `.zndsl` file with one natural-language assertion per line, run with the `zombienet test` command of the [Zombienet CLI](https://github.com/paritytech/zombienet).
- **Zombienet SDK**: The [`zombienet-sdk`](https://github.com/paritytech/zombienet-sdk) Rust crate, which builds a network configuration in code, spawns it, and exposes each node so you can assert on it from regular Rust tests.

This guide covers both, using a local network with the `rococo-local` relay chain spec that is built into the `polkadot` binary. For the full list of network configuration options, see the [Zombienet reference](/tools/zombienet.md#configure-zombienet). To test XCM logic without spawning nodes, see [Test XCM with the Emulator and Simulator](/polkadot-sdk/testing/test-xcm.md).

## Prerequisites

Before you begin, ensure you have the following:

- The `polkadot`, `polkadot-prepare-worker`, `polkadot-execute-worker`, and `polkadot-parachain` binaries from the [Polkadot SDK releases](https://github.com/paritytech/polkadot-sdk/releases/tag/{{ dependencies.repositories.polkadot_sdk.version }}) in your `PATH`. The [native provider](/tools/zombienet.md#native) runs these binaries as local processes.
- For DSL tests, the [Zombienet CLI](/tools/zombienet.md#install-zombienet).
- For SDK tests, [Rust](https://rust-lang.org/tools/install/) and Cargo.

## Write a Test with the Zombienet DSL

A DSL test consists of a network configuration file and a `.zndsl` test file that points to it. When you run the test, Zombienet spawns the network, runs each assertion in order, reports the result of each, and then saves the node logs and tears the network down.

### Define the Network

Create a network configuration. The following file is the [basic configuration](/tools/zombienet.md#basic-configuration) from the Zombienet reference: two `rococo-local` validators and one collator for parachain 100.

```toml title="basic-network.toml"
[settings]
timeout = 1000

[relaychain]
chain = "rococo-local"
default_command = "polkadot"

    [[relaychain.nodes]]
    name = "alice"
    validator = true

    [[relaychain.nodes]]
    name = "bob"
    validator = true

[[parachains]]
id = 100

    [parachains.collator]
    name = "collator-01"
    command = "polkadot-parachain"
```

### Create the Test File

A `.zndsl` file starts with header fields, followed by one assertion or command per line. The [test DSL reference](https://paritytech.github.io/zombienet/cli/test-dsl-definition-spec.html) defines three header fields:

- **`Description`**: Optional. A description of the test suite.
- **`Network`**: Required. The path to the network configuration file, in JSON or TOML. The DSL grammar expects a path, so start relative paths with `./`.
- **`Creds`**: Optional. The credentials file for the Kubernetes provider, or `config` to use the default. Only the Kubernetes provider uses it.

Create a test file next to `basic-network.toml`:

```text title="0001-basic-network.zndsl"
Description: Basic network smoke test
Network: ./basic-network.toml
Creds: config

# Nodes are running
alice: is up
bob: is up
collator-01: is up

# The parachain is registered and producing blocks
alice: parachain 100 is registered within 225 seconds
alice: parachain 100 block height is at least 10 within 250 seconds

# Metrics and logs
alice: reports node_roles is 4
alice: reports block height is at least 5 within 60 seconds
bob: log line matches glob "*rted #1*" within 10 seconds
```

Zombienet uses the file name as the test name, without the leading number and the first `-`. The test above is reported as `basic-network`. Lines that start with `#` or `//` are comments.

### Add Assertions

Each assertion starts with the name of the node it runs against, followed by a colon. Most assertions accept an optional `within X seconds` suffix that retries the check until it passes or the time runs out. The following assertions are defined in the DSL reference and the [DSL grammar](https://github.com/paritytech/zombienet/blob/{{ dependencies.repositories.zombienet.version }}/crates/parser/src/zombienet.pest):

- **Node is up**: `alice: is up`
- **Parachain is registered**: `alice: parachain 100 is registered within 225 seconds`
- **Parachain block height**: `alice: parachain 100 block height is at least 10 within 250 seconds`
- **Metric value**: `alice: reports node_roles is 4`
- **Metric arithmetic**: `alice: reports block height minus finalised block is lower than 10 within 20 seconds`
- **Histogram**: `alice: reports histogram polkadot_pvf_execution_time has at least 2 samples in buckets ["0.1", "0.25", "0.5", "+Inf"] within 100 seconds`
- **Log line**: `bob: log line matches "Imported #[0-9]+" within 10 seconds`
- **Log line count**: `alice: count of log lines matching glob "*rted #1*" is 1 within 10 seconds`
- **System event**: `bob: system event contains "A candidate was included" within 20 seconds`
- **Trace**: `alice: trace with traceID 94c1501a78a0d83c498cc92deec264d9 contains ["answer-chunk-request", "answer-chunk-request"]`
- **JavaScript script**: `alice: js-script ./check-validators.js return is greater than 1 within 200 seconds`
- **TypeScript script**: `alice: ts-script ./check-validators.ts return is greater than 1 within 200 seconds`
- **Runtime upgrade**: `alice: parachain 100 perform upgrade with ./runtime.compact.compressed.wasm within 200 seconds`

Metric assertions read the node's Prometheus metrics. Besides metric names such as `node_roles`, the grammar accepts the aliases `block height` (or `best block`), `finalised height` (or `finalised block`), and `peers count`. Comparisons can be written as words (`is`, `is at least`, `is greater than`, `is lower than`) or symbols (`==`, `>=`, `>`, `<`, `<=`, `!=`).

Log assertions match a regular expression by default. Add `glob` before the pattern to use a glob instead. System event assertions subscribe to events when the assertion starts, so they cannot match events emitted before that point. The DSL reference notes that trace assertions are not supported with the native provider.

### Run Custom Scripts

When the built-in assertions are not enough, a `js-script` or `ts-script` step runs your own code against a node. The script exports a `run` function, which receives the node name, information about the running network, and optional arguments. The return value can be compared with `return is ...`. While the script runs, Zombienet provides a global `zombie` object that includes `connect`, which opens a [Polkadot.js API](https://polkadot.js.org/docs/api) connection, and the `ApiPromise`, `Keyring`, and `WsProvider` classes.

The following script, adapted from the [`check-balance.js`](https://github.com/paritytech/zombienet/blob/{{ dependencies.repositories.zombienet.version }}/tests/check-balance.js) test in the Zombienet repository, returns the free balance of a development account:

```js title="check-balance.js"
const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

async function run(nodeName, networkInfo, args) {
  const { wsUri, userDefinedTypes } = networkInfo.nodesByName[nodeName];
  const api = await zombie.connect(wsUri, userDefinedTypes);

  const account = args[0] === 'alice' ? ALICE : BOB;
  const balance = await api.query.system.account(account);
  return balance.data.free;
}

module.exports = { run };
```

Pass arguments with `with`. Zombienet splits a string argument on commas into the `args` array:

```text
alice: js-script ./check-balance.js with "alice" return is greater than 0 within 30 seconds
```

For `js-script`, you can instead pass a single-quoted JSON object, which Zombienet parses and passes as `args`. `ts-script` steps take only double-quoted arguments. The grammar also defines a `run` step, `alice: run ./script.sh with "arg1,arg2" return is 0`, which runs a shell script.

### Use Commands

The DSL reference also defines commands that act on the network instead of asserting on it, and notes that they work only with the Podman and Kubernetes providers:

- **`alice: restart after 5 seconds`**: Stops the node's process and starts it again, immediately or after the given delay.
- **`alice: pause`**: Pauses the node's process.
- **`alice: resume`**: Resumes a paused node.
- **`sleep 10 seconds`**: Pauses the test runner.

### Run the Test

Run the test with `zombienet test`, choosing a provider with `--provider`. Without the flag, the test runner uses the Kubernetes provider.

```bash
zombienet test 0001-basic-network.zndsl --provider native
```

Zombienet spawns the network described in the `Network` header, runs each line in order, and prints a pass or fail result for each assertion. If any assertion fails, the command exits with a non-zero status, which makes it usable as a CI step.

To run the assertions against a network that is already running, pass the path to that network's spec JSON as a second argument after the test file.

## Write Tests with the Zombienet SDK

[`zombienet-sdk`](https://docs.rs/zombienet-sdk/{{ dependencies.crates.zombienet_sdk.version }}/zombienet_sdk/) is a Rust library for building, spawning, and interacting with Zombienet networks. It supports the native, Docker, and Kubernetes providers. Because tests are regular Rust tests, you can combine network assertions with [Subxt](/tools/subxt.md) queries and transactions in the same test.

### Add the Dependency

Add `zombienet-sdk` and an async runtime to the crate that holds your tests. This guide uses version {{ dependencies.crates.zombienet_sdk.version }} from [crates.io](https://crates.io/crates/zombienet-sdk). The `tokio` features enable the `#[tokio::test]` macro with a multi-threaded runtime, and `futures` provides `StreamExt` for the block subscription used later in this guide.

```toml title="Cargo.toml"
[dev-dependencies]
zombienet-sdk = "{{ dependencies.crates.zombienet_sdk.version }}"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
futures = "0.3"
```

### Build the Network Configuration

`NetworkConfigBuilder` describes the same topology as `basic-network.toml` in code. Each `with_*` method takes a closure that configures one part of the network:

```rust title="tests/network.rs"
use zombienet_sdk::{NetworkConfig, NetworkConfigBuilder, NetworkConfigExt};

fn basic_network() -> NetworkConfig {
    NetworkConfigBuilder::new()
        .with_relaychain(|r| {
            r.with_chain("rococo-local")
                .with_default_command("polkadot")
                .with_validator(|node| node.with_name("alice"))
                .with_validator(|node| node.with_name("bob"))
        })
        .with_parachain(|p| {
            p.with_id(100)
                .with_collator(|n| n.with_name("collator-01").with_command("polkadot-parachain"))
        })
        .build()
        .unwrap()
}
```

`build` validates the configuration and returns the list of errors if any setting is invalid. To reuse an existing TOML file instead, load it with `NetworkConfig::load_from_toml("./basic-network.toml")`.

### Spawn the Network and Assert

The `NetworkConfigExt` trait adds `spawn_native`, `spawn_docker`, and `spawn_k8s` to `NetworkConfig`. Each returns a `Network`, from which you get nodes by name with `get_node`. The following test is based on the SDK's [`smoke-native.rs`](https://github.com/paritytech/zombienet-sdk/blob/v{{ dependencies.crates.zombienet_sdk.version }}/crates/sdk/tests/smoke-native.rs) test and [`simple_network_example.rs`](https://github.com/paritytech/zombienet-sdk/blob/v{{ dependencies.crates.zombienet_sdk.version }}/crates/examples/examples/simple_network_example.rs) example:

```rust title="tests/network.rs"
use futures::StreamExt;
use zombienet_sdk::subxt;

#[tokio::test(flavor = "multi_thread")]
async fn basic_network_produces_blocks() {
    let network = basic_network().spawn_native().await.unwrap();

    // Wait until every node reports that it is running.
    network.wait_until_is_up(60).await.unwrap();

    // Relay chain: wait for the best block to pass 9.
    let alice = network.get_node("alice").unwrap();
    alice
        .wait_metric_with_timeout("block_height{status=\"best\"}", |height| height > 9_f64, 120u64)
        .await
        .unwrap();

    // Parachain: connect with Subxt and wait for three finalized blocks.
    let client = network
        .get_node("collator-01")
        .unwrap()
        .wait_client::<subxt::PolkadotConfig>()
        .await
        .unwrap();
    let mut blocks = client.blocks().subscribe_finalized().await.unwrap().take(3);
    while let Some(block) = blocks.next().await {
        println!("Parachain block #{}", block.unwrap().header().number);
    }

    network.destroy().await.unwrap();
}
```

`NetworkNode` provides the following methods for assertions:

- **`wait_until_is_up`**: Waits until the node reports that it is running. `Network` has a method with the same name that waits for every node.
- **`reports`**: Returns the current value of a Prometheus metric.
- **`assert` and `assert_with`**: Compare a metric's current value with a number or a predicate.
- **`wait_metric` and `wait_metric_with_timeout`**: Poll a metric until a predicate returns `true`, optionally with a timeout in seconds.
- **`wait_log_line_count`**: Waits until the node's logs contain a pattern a given number of times. The second argument selects glob matching instead of a substring.
- **`logs`**: Returns the node's logs.
- **`wait_client`**: Returns a Subxt `OnlineClient` connected to the node once its RPC endpoint is ready. Use `ws_uri` to get the endpoint for another client.
- **`pause`, `resume`, and `restart`**: Control the node's process.

`Network` also lets you change the network while it runs, with `add_node`, `add_collator`, `add_parachain`, and `register_parachain`. See the [`NetworkNode`](https://docs.rs/zombienet-sdk/{{ dependencies.crates.zombienet_sdk.version }}/zombienet_sdk/struct.NetworkNode.html) and [`Network`](https://docs.rs/zombienet-sdk/{{ dependencies.crates.zombienet_sdk.version }}/zombienet_sdk/struct.Network.html) API docs and the [SDK examples](https://github.com/paritytech/zombienet-sdk/tree/v{{ dependencies.crates.zombienet_sdk.version }}/crates/examples/examples) for these methods.

### Choose the Provider at Run Time

To run the same tests with different providers, replace `spawn_native` with the spawn function from `zombienet_sdk::environment::get_spawn_fn`. It reads the `ZOMBIE_PROVIDER` environment variable, which accepts `native`, `k8s`, or `docker`, and uses Docker when the variable is unset or has any other value:

```rust
use zombienet_sdk::environment::get_spawn_fn;

let spawn_fn = get_spawn_fn();
let network = spawn_fn(basic_network()).await.unwrap();
```

Then select the provider when you run the tests:

```bash
ZOMBIE_PROVIDER=native cargo test --test network -- --nocapture
```

## Run Tests in CI

Both kinds of tests need the node binaries on the CI runner. The zombienet-sdk repository's own [integration workflow](https://github.com/paritytech/zombienet-sdk/blob/v{{ dependencies.crates.zombienet_sdk.version }}/.github/workflows/ci_integration.yml) downloads them from a Polkadot SDK release, adds them to `PATH`, sets `ZOMBIE_PROVIDER=native`, and runs the test binary. The following script follows the same steps on a Linux x86-64 runner:

```bash
POLKADOT_SDK_VERSION={{ dependencies.repositories.polkadot_sdk.version }}
mkdir -p bin
for bin in polkadot polkadot-prepare-worker polkadot-execute-worker polkadot-parachain; do
  curl --retry 3 -L -o bin/$bin \
    https://github.com/paritytech/polkadot-sdk/releases/download/$POLKADOT_SDK_VERSION/$bin
  chmod 755 bin/$bin
done
export PATH=$(pwd)/bin:$PATH

# Zombienet SDK tests
ZOMBIE_PROVIDER=native cargo test --test network -- --nocapture

# Zombienet DSL tests
curl -L -o bin/zombienet \
  https://github.com/paritytech/zombienet/releases/download/{{ dependencies.repositories.zombienet.version }}/zombienet-linux-x64
chmod 755 bin/zombienet
zombienet test 0001-basic-network.zndsl --provider native
```

Keep the following in mind when you run network tests in CI:

- Spawning a network and waiting for parachain blocks takes minutes. Set generous timeouts in your assertions and in the CI job.
- Upload the Zombienet log files as artifacts when a job fails. The zombienet-sdk workflow collects the `*.log` files from the `/tmp/zombie*` directories that the native provider creates.
- Pin the binary versions. The release you download determines the runtime and node behavior your tests exercise.

## Where to Go Next

<div class="grid cards" markdown>

-   __Zombienet Reference__

    ---

    Look up installation methods, providers, and every relay chain, parachain, and collator configuration option.

    [:octicons-arrow-right-24: Zombienet reference](/tools/zombienet.md)

-   __Run a Parachain Network__

    ---

    Build a parachain from the template and run it with a local relay chain using Zombienet.

    [:octicons-arrow-right-24: Run a parachain network](/polkadot-sdk/testing/run-a-parachain-network.md)

-   __Test XCM with the Emulator and Simulator__

    ---

    Test cross-chain messages in a single Rust process, without spawning nodes.

    [:octicons-arrow-right-24: Test XCM](/polkadot-sdk/testing/test-xcm.md)

</div>
