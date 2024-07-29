use alloy_consensus::SignableTransaction;
use alloy_primitives::{Address, Bytes, TxKind, U256};
use alloy_rlp::{Decodable, RlpDecodable};
use alloy_rpc_client::{ClientBuilder, ReqwestClient, RpcCall};
use alloy_signer::Signature;
use aws_sdk_kms::operation::sign;
use serde::Serialize;
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

#[derive(Debug, RlpDecodable)]
struct SignTransactionResponse {
    // TODO: fill with tx fields
    r: U256,
    s: U256,
    parity: u8,
}

impl Web3Signer {
    /// TODO: add docs
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
            from: self.address.to_string(),
            to: tx.to(),
            gas: format!("0x{:x}", tx.gas_limit()),
            gas_price: format!("0x{:x}", tx.gas_price().unwrap()),
            nonce: format!("0x{:x}", tx.nonce()),
            data: Bytes::copy_from_slice(tx.input()).to_string(),
        };

        let request: RpcCall<_, Vec<SignTransactionParams>, Bytes> =
            self.client.request("eth_signTransaction", vec![params]);

        let mut rlp_encoded_signed_tx = request.await.unwrap();
        let signed_tx =
            SignTransactionResponse::decode(&mut rlp_encoded_signed_tx.as_ref()).unwrap(); // TODO: fix this
        Signature::from_rs_and_parity(signed_tx.r, signed_tx.s, signed_tx.parity as u64)
            .map_err(alloy_signer::Error::from)
    }
}
