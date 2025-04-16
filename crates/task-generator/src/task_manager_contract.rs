use alloy_sol_types::SolType;

use crate::task::Task;
use crate::task_response::TaskResponse;

pub trait TaskManagerContract<T: SolType, R: SolType, N: SolType> {
    /// Type for task indices
    //type Index;

    /// Type for inputs of each task
    type Input;

    /// Type for outputs of each task
    type Output;

    fn create_new_task(&self, task: Task<Self::Input>) -> Task<Self::Input>;

    fn respond_to_task(
        &self,
        task: T,
        response: R,
        non_signer_stakes_and_signature: N,
    ) -> TaskResponse<R>;

    /*pub async fn send_aggregated_response(
        &self,
        task: Task<Self::Input>,
        task_response: TaskResponse,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> Result<(), ChainIoError> {
        let pr = get_signer(&self.signer, &self.rpc_url);
        let task_manager_contract = IncredibleSquaringTaskManager::new(self.task_manager_addr, pr);
        let receipt = task_manager_contract
            .respondToTask(task, task_response, non_signer_stakes_and_signature)
            .send()
            .await?
            .get_receipt()
            .await?;
        info!("receipt for response: {:?}", receipt.transaction_hash);

        Ok(())
    }
    */
    // fn submit_challenge(&self) ->
}
