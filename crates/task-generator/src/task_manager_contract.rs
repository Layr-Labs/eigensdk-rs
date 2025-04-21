use alloy::dyn_abi::SolType;

#[allow(missing_docs)]
pub trait TaskManagerContract<T: SolType, R: SolType, N: SolType> {
    type Event;

    fn create_new_task(&self, task: T);

    fn respond_to_task(&self, task: T, response: R, non_signer_stakes_and_signature: N) {
        todo!("Implement respond_to_task")
    }

    fn receive_event() -> Result<Self::Event, ()> {
        todo!("Implement receive_event")
    }

    // fn submit_challenge(&self) ->
}
