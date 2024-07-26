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
#[allow(non_snake_case)]
struct SignTransactionParams {
    from: String,
    to: TxKind,
    gas: String,
    gasPrice: String,
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
            gasPrice: format!("0x{:x}", tx.gas_price().unwrap()),
            nonce: format!("0x{:x}", tx.nonce()),
            data: Bytes::copy_from_slice(tx.input()).to_string(),
        };

        let request: RpcCall<_, Vec<SignTransactionParams>, SignTransactionResponse> =
            self.client.request("eth_signTransaction", vec![params]);

        let res = request.await.unwrap();

        Signature::from_rs_and_parity(res.r, res.s, res.parity).map_err(alloy_signer::Error::from)
    }
}
