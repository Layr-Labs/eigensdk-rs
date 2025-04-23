use std::time::Duration;

use crate::{non_signer::NonSignerStakesAndSignature, task::Task, task_response::TaskResponse};
use eigen_services_blsaggregation::bls_agg::TaskMetadata;

pub trait TaskManagerContract {
    // /// Type for task indices
    // type Index = u32;

    /// Type for inputs of each task
    type Input: Clone;

    /// Type for outputs of each task
    type Output: Clone;

    /// New task event
    type NewTaskEvent;

    // DAMIAN:
    // We have a problem with these methods: they are very tightly coupled to
    // the bindings. For example, `create_new_task` from the binding expects
    // an `IncredibleSquaringTaskManager::Task`, and the same applies to
    // `TaskResponse`. If we want to use this interface, the user should
    // re-create the `IncredibleSquaringTaskManager::Task` using the values
    // of our Task struct. I think it feels weird.

    fn create_new_task(&self, task: Task<Self::Input>);

    fn respond_to_task(
        &self,
        task: Task<Self::Input>,
        response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    );

    fn process_new_task(
        &self,
        event: Self::NewTaskEvent,
        task_timeout: Duration,
        window_duration: Duration,
    ) -> TaskMetadata;

    fn task_response_to_bytes(&self, response: TaskResponse<Self::Output>) -> &[u8];
}
