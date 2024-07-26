use alloy_consensus::SignableTransaction;
use alloy_primitives::{Address, Bytes, TxKind, U256};
use alloy_rpc_client::{ClientBuilder, ReqwestClient, RpcCall};
use alloy_signer::Signature;
use serde::{Deserialize, Serialize};
use url::Url;

/// A signer that sends an rpc request to sign a transaction remotely
#[derive(Debug)]
pub struct Web3Signer {
    /// Client used to send an RPC request
    pub client: ReqwestClient,
    /// Address of the account that intends to sign a transaction.
    /// It must match the `from` field in the transaction.
    pub address: Address,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct SignTransactionParams {
    from: String,
    to: TxKind,
    gas: String,
    gas_price: String,
    nonce: String,
    data: String,
}

#[derive(Deserialize, Clone, Debug)]
struct SignTransactionResponse {
    r: U256,
    s: U256,
    parity: bool,
}

impl Web3Signer {
    /// TODO: add docs
    pub fn new(address: Address, url: Url) -> Self {
        Web3Signer {
            client: ClientBuilder::default().http(url),
            address,
        }
    }

    /// TODO: implement alloy TxSigner trait
    pub async fn sign_transaction(
        &self,
        tx: &mut dyn SignableTransaction<Signature>,
    ) -> alloy_signer::Result<Signature, alloy_signer::Error> {
        let params = SignTransactionParams {
            from: self.address.to_string(),
            to: tx.to(),
            gas: format!("0x{:x}", tx.gas_limit()),
            gas_price: format!("0x{:x}", tx.gas_price().unwrap()),
            nonce: format!("0x{:x}", tx.nonce()),
            data: Bytes::copy_from_slice(tx.input()).to_string(),
        };

        let request: RpcCall<_, Vec<SignTransactionParams>, SignTransactionResponse> =
            self.client.request("eth_signTransaction", vec![params]);

        let res = request.await.unwrap();

        Signature::from_rs_and_parity(res.r, res.s, res.parity).map_err(alloy_signer::Error::from)
    }
}
