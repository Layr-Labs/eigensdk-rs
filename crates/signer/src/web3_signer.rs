use alloy_consensus::SignableTransaction;
use alloy_network::TxSigner;
use alloy_primitives::{Address, Bytes, TxKind};
use alloy_rpc_client::{ClientBuilder, ReqwestClient, RpcCall};
use alloy_signer::k256::ecdsa::Signature;
use async_trait::async_trait;
use serde::Serialize;
use url::Url;

pub struct Web3Signer {
    client: ReqwestClient,
    address: Address,
}

impl Web3Signer {
    pub fn new(address: Address, url: Url) -> Self {
        Web3Signer {
            client: ClientBuilder::default().http(url),
            address,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
struct SignTransactionParams {
    from: Address,
    to: TxKind,
    gas: u128,
    gas_price: Option<u128>,
    nonce: u64,
    data: Bytes,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TxSigner<Signature> for Web3Signer {
    fn address(&self) -> Address {
        self.address
    }
    async fn sign_transaction(
        &self,
        tx: &mut dyn SignableTransaction<Signature>,
    ) -> alloy_signer::Result<Signature, alloy_signer::Error> {
        let params = SignTransactionParams {
            from: self.address,
            to: tx.to(),
            gas: tx.gas_limit(),
            gas_price: tx.gas_price(),
            nonce: tx.nonce(),
            data: Bytes::copy_from_slice(tx.input()),
        };

        let request: RpcCall<_, SignTransactionParams, Bytes> =
            self.client.request("eth_signTransaction", params);

        let res = request.await.unwrap();

        Signature::from_slice(&res).map_err(|e| alloy_signer::Error::Ecdsa(e))
    }
}
