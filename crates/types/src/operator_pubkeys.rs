use alloy::primitives::U256;
use ark_ff::PrimeField;
use eigen_crypto_bls::{BlsG1Point, BlsG2Point, BlsKeyPair};
use eigen_utils::slashing::middleware::blsapkregistry::BN254::{G1Point, G2Point};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorPubKeys {
    pub g1_pub_key: BlsG1Point,
    pub g2_pub_key: BlsG2Point,
}

impl From<BlsKeyPair> for OperatorPubKeys {
    fn from(keypair: BlsKeyPair) -> Self {
        Self {
            g1_pub_key: keypair.public_key(),
            g2_pub_key: keypair.public_key_g2(),
        }
    }
}

impl OperatorPubKeys {
    pub fn to_contract_public_keys(&self) -> (G1Point, G2Point) {
        let x = self.g1_pub_key.g1().x;
        let y = self.g1_pub_key.g1().y;
        let g1 = G1Point {
            X: U256::from_limbs(x.into_bigint().0),
            Y: U256::from_limbs(y.into_bigint().0),
        };

        let x2 = self.g2_pub_key.g2().x;
        let y2 = self.g2_pub_key.g2().y;
        let g2 = G2Point {
            X: [
                U256::from_limbs(x2.c0.into_bigint().0),
                U256::from_limbs(x2.c1.into_bigint().0),
            ],
            Y: [
                U256::from_limbs(y2.c0.into_bigint().0),
                U256::from_limbs(y2.c1.into_bigint().0),
            ],
        };
        (g1, g2)
    }
}

#[cfg(test)]
mod tests {
    use super::OperatorPubKeys;
    use alloy::primitives::U256;
    use ark_bn254::{G1Affine, G2Affine};
    use eigen_crypto_bls::{BlsG1Point, BlsG2Point};

    #[test]
    fn test_operator_pub_keys() {
        let g1_pub_key = BlsG1Point::new(G1Affine {
            x: ark_ff::Fp::from(1),
            y: ark_ff::Fp::from(1),
            infinity: false,
        });

        let g2_pub_key = BlsG2Point::new(G2Affine {
            x: ark_ff::QuadExtField {
                c0: ark_ff::Fp::from(1),
                c1: ark_ff::Fp::from(1),
            },
            y: ark_ff::QuadExtField {
                c0: ark_ff::Fp::from(1),
                c1: ark_ff::Fp::from(1),
            },
            infinity: false,
        });

        let operator1_pubkeys = OperatorPubKeys {
            g1_pub_key,
            g2_pub_key,
        };
        let (g1, g2) = operator1_pubkeys.to_contract_public_keys();

        // assert g1
        let x_contract = g1.X;
        let y_contract = g1.Y;
        assert_eq!(x_contract, U256::from(1));
        assert_eq!(y_contract, U256::from(1));

        // assert g2
        let x_g2_contract_0 = g2.X[0];
        let x_g2_contract_1 = g2.X[1];
        let y_g2_contract_0 = g2.Y[0];
        let y_g2_contract_1 = g2.Y[1];

        assert_eq!(x_g2_contract_0, U256::from(1));
        assert_eq!(x_g2_contract_1, U256::from(1));
        assert_eq!(y_g2_contract_0, U256::from(1));
        assert_eq!(y_g2_contract_1, U256::from(1));
    }
}
