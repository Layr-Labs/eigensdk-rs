use alloy_consensus::SignableTransaction;
use alloy_primitives::{Address, TxKind, U256};
use alloy_rpc_client::{ClientBuilder, ReqwestClient, RpcCall};
use alloy_signer::Signature;
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
struct SignParams<'a> {
    from: Address,
    to: TxKind,
    gas: u128,
    gas_price: Option<u128>,
    nonce: u64,
    data: &'a [u8],
}

trait Signer {
    async fn sign_transaction(
        &self,
        tx: &mut dyn SignableTransaction<Signature>,
    ) -> alloy_signer::Result<Signature>;
}

impl Signer for Web3Signer {
    async fn sign_transaction(
        &self,
        tx: &mut dyn SignableTransaction<Signature>,
    ) -> alloy_signer::Result<Signature> {
        let params = SignParams {
            from: self.address,
            to: tx.to(),
            gas: tx.gas_limit(),
            gas_price: tx.gas_price(),
            nonce: tx.nonce(),
            data: tx.input(),
        };

        let request: RpcCall<_, SignParams, U256> =
            self.client.request("eth_signTransaction", params);

        request.await.unwrap();

        // TODO: return signature from response
    }
}
