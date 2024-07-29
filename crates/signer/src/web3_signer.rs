use alloy_consensus::{SignableTransaction, TxLegacy};
use alloy_primitives::{Address, Bytes, TxKind, U256};
use alloy_rpc_client::{ClientBuilder, ReqwestClient, RpcCall};
use alloy_signer::Signature;
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
    value: U256,
    gas: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    gas_price: Option<String>,
    nonce: String,
    data: String,
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
            from: self.address.to_string(),
            to: tx.to(),
            value: tx.value(),
            gas: format!("0x{:x}", tx.gas_limit()),
            gas_price: tx.gas_price().map(|price| format!("0x{:x}", price)),
            nonce: format!("0x{:x}", tx.nonce()),
            data: Bytes::copy_from_slice(tx.input()).to_string(),
        };

        let request: RpcCall<_, Vec<SignTransactionParams>, Bytes> =
            self.client.request("eth_signTransaction", vec![params]);
        let rlp_encoded_signed_tx = request.await.map_err(alloy_signer::Error::other)?;

        let signed_tx = TxLegacy::decode_signed_fields(&mut rlp_encoded_signed_tx.as_ref())
            .map_err(alloy_signer::Error::other)?;

        Ok(*signed_tx.signature())
    }
}
