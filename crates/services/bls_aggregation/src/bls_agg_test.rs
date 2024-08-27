#[cfg(test)]
pub mod integration_test {
    use alloy_node_bindings::Anvil;
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_types::operator::operator_id_from_g1_pub_key;

    #[test]
    fn test_1_quorum_1_operator() {
        let _anvil = Anvil::new().try_spawn().unwrap();

        let bls_key_pair = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let _operator_id = operator_id_from_g1_pub_key(bls_key_pair.public_key());

        // TODO: integration tests need a `chainio/client` builder to construct the clients.
        // Will implement this builder and integration test in a following PR.
    }
}
