#![cfg(target_arch = "wasm32")]

use subxt::lightclient::{ChainConfig, LightClient};
use subxt::{OnlineClient, PolkadotConfig};
use wasm_bindgen_test::*;

async fn connect() -> OnlineClient<PolkadotConfig> {
    let config = ChainConfig::chain_spec(include_str!("../local.json"))
        .set_bootnodes([
            "/ip4/127.0.0.1/tcp/9944/ws/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp",
            "/ip4/127.0.0.1/tcp/9945/ws/p2p/12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD",
        ])
        .unwrap();
    let (_client, rpc) = LightClient::relay_chain(config).unwrap();
    // TODO: doesn't work
    OnlineClient::<PolkadotConfig>::from_rpc_client(rpc)
        .await
        .unwrap()
}

wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
async fn testing() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    // Start substrate node alice:
    //
    // $ /target/release/substrate-node \
    // --base-path /tmp/alice \
    // --chain local \
    // --alice \
    // --port 30333 \
    // --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    // --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
    // --validator

    // Start substrate node bob:
    //
    // $ ./target/release/substrate-node \
    // --base-path /tmp/bob \
    // --chain local \
    // --bob \
    // --port 30334 \
    // --rpc-port 9945 \
    // --node-key 0000000000000000000000000000000000000000000000000000000000000002 \
    // --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
    // --validator

    // Connect to the chain.
    connect().await;

    assert_eq!(1, 0);
}
