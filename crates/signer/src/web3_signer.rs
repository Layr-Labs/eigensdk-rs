use alloy_consensus::SignableTransaction;
use alloy_primitives::{Address, Bytes, TxKind, U256};
use alloy_rpc_client::{ClientBuilder, ReqwestClient, RpcCall};
use alloy_signer::Signature;
use serde::{Deserialize, Serialize};
use url::Url;

pub struct Web3Signer {
    pub client: ReqwestClient,
    pub address: Address,
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

#[derive(Deserialize, Clone, Debug)]
struct SignTransactionResponse {
    r: U256,
    s: U256,
    parity: bool,
}

impl Web3Signer {
    pub fn new(address: Address, url: Url) -> Self {
        Web3Signer {
            client: ClientBuilder::default().http(url),
            address,
        }
    }

    // TODO: implement alloy TxSigner trait
    pub async fn sign_transaction(
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

        let request: RpcCall<_, SignTransactionParams, SignTransactionResponse> =
            self.client.request("eth_signTransaction", params);

        let res = request.await.unwrap();

        Signature::from_rs_and_parity(res.r, res.s, res.parity)
            .map_err(|e| alloy_signer::Error::from(e))
    }
}
