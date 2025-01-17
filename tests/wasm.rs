#![cfg(target_arch = "wasm32")]

use std::sync::Arc;
use subxt::backend::chain_head::*;
use subxt::lightclient::{ChainConfig, LightClient};
use subxt::{OnlineClient, PolkadotConfig};
use wasm_bindgen_test::*;

#[allow(unused)]
async fn connect() -> OnlineClient<PolkadotConfig> {
    let config = ChainConfig::chain_spec(include_str!("../local.json"))
        .set_bootnodes([
            "/ip4/127.0.0.1/tcp/30333/ws/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp",
            "/dns/localhost/tcp/30333/ws/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp",
        ])
        .unwrap();
    let (_client, rpc) = LightClient::relay_chain(config).unwrap();
    // TODO: doesn't work
    OnlineClient::<PolkadotConfig>::from_rpc_client(rpc)
        .await
        .unwrap()
}

async fn connect_chainhead() -> OnlineClient<PolkadotConfig> {
    let config = ChainConfig::chain_spec(include_str!("../local.json"))
        .set_bootnodes([
            "/ip4/127.0.0.1/tcp/30333/ws/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp",
            "/dns/localhost/tcp/30333/ws/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp",
        ])
        .unwrap();
    let (_client, rpc) = LightClient::relay_chain(config).unwrap();
    let backend: ChainHeadBackend<PolkadotConfig> =
        ChainHeadBackendBuilder::default().build_with_background_driver(rpc);
    // TODO: doesn't work
    OnlineClient::<PolkadotConfig>::from_backend(Arc::new(backend))
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
    connect_chainhead().await;

    assert_eq!(1, 0);
}
