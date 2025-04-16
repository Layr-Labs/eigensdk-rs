use alloy::dyn_abi::SolType;

#[allow(missing_docs)]
pub trait TaskManagerContract<T: SolType, R: SolType, N: SolType> {
    fn create_new_task(&self, task: T);

    fn respond_to_task(&self, task: T, response: R, non_signer_stakes_and_signature: N);

    // fn submit_challenge(&self) ->
}
