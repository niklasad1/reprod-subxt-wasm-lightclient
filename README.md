### Re-produce of 

### How to run the code

Terminal #1

```bash
$ ./target/release/substrate-node \
--base-path /tmp/alice \
--chain local \
--alice \
--port 30333 \
--rpc-port 9945 \
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator
```

Terminal #2

```bash
$ ./target/release/substrate-node \
--base-path /tmp/bob \
--chain local \
--bob \
--port 30334 \
--rpc-port 9944 \
--node-key 0000000000000000000000000000000000000000000000000000000000000002 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator
```

Terminal #3

```bash
$ export RUST_LOG=debug
$ wasm-pack test --firefox --headless
```