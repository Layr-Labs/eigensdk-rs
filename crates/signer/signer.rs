use alloy_primitives::Address;
use k256::ecdsa::SigningKey;

#[derive(PartialEq, Debug)]
pub struct BasicSigner {
    private_key: SigningKey,
    account_address: Address,
}

impl BasicSigner {
    pub fn new(private_key: SigningKey) -> Self {
        let account_address = Address::from_private_key(&private_key);
        BasicSigner {
            private_key,
            account_address,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::BasicSigner;
    use alloy_primitives::Address;
    use hex_literal::hex;
    use k256::ecdsa::SigningKey;

    #[test]
    fn signer_from_private_key() {
        let signing_key = SigningKey::from_bytes(
            &hex!("4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318").into(),
        )
        .unwrap();
        let expected_address =
            Address::from_slice(&hex::decode("2c7536E3605D9C16a7a3D7b1898e529396a65c23").unwrap());
        let basic_signer = BasicSigner::new(signing_key);

        assert_eq!(basic_signer.account_address, expected_address);
    }
}
