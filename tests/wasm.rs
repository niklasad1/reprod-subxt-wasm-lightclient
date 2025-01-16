#![cfg(target_arch = "wasm32")]

use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use futures::stream::{Stream, StreamExt, TryStreamExt};
use serde_json::value::RawValue;
use subxt::backend::rpc::{RawRpcFuture, RawRpcSubscription};
use subxt::{OnlineClient, PolkadotConfig};
use wasm_bindgen_test::*;

#[derive(Clone)]
pub struct CustomRpc {
    rpc: subxt_lightclient::LightClientRpc,
}

impl subxt::backend::rpc::RpcClientT for CustomRpc {
    // copied from this repo
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RawRpcFuture<'a, Box<RawValue>> {
        Box::pin(
            self.rpc
                .request(method.to_string(), params)
                .map_err(|e| subxt::error::RpcError::ClientError(Box::new(e))),
        )
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        Box::pin(async {
            let sub = self
                .rpc
                .subscribe(sub.to_string(), params, unsub.to_string())
                .await
                .map_err(|e| subxt::error::RpcError::ClientError(Box::new(e)))?;

            Ok(RawRpcSubscription {
                id: Some(sub.id().to_string()),
                stream: sub
                    .map_err(|e| subxt::error::RpcError::ClientError(Box::new(e)))
                    .boxed(),
            })
        })
    }
}

async fn connect() -> OnlineClient<PolkadotConfig> {
    // TODO: runs forever in wasm. doesn't produce any error or anything. it's just stuck
    let config = subxt_lightclient::ChainConfig::chain_spec(include_str!("../local.json"))
        .set_bootnodes([
            "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp",
        ])
        .unwrap();
    let (_client, rpc) = subxt_lightclient::LightClient::relay_chain(config).unwrap();
    let custom_rpc = CustomRpc { rpc };
    OnlineClient::<PolkadotConfig>::from_rpc_client(custom_rpc)
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
    // --port)) 30334 \
    // --rpc-port 9945 \
    // --node-key 0000000000000000000000000000000000000000000000000000000000000002 \
    // --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
    // --validator

    // Connect to the chain.
    connect().await;

    assert_eq!(1, 0);
}
