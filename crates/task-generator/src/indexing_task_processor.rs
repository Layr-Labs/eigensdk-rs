use crate::task::Task;
use crate::task_manager_contract::TaskManagerContract;
use crate::task_response::TaskResponse;
use alloy_sol_types::SolType;
use eigen_types::avs::{TaskIndex, TaskResponseDigest};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct IndexingTaskProcessor<
    TM: TaskManagerContract<INPUT, R, G1Points, G2Points>,
    INPUT: SolType,
    R: SolType,
    G1Points: alloy::sol_types::SolType + Clone,
    G2Points: alloy::sol_types::SolType + Clone,
> where
    TM::Input: Debug,
    <TM as TaskManagerContract<INPUT, R, G1Points, G2Points>>::Output: alloy_sol_types::SolType,
{
    /// Hashmap to store the created tasks
    tasks: HashMap<TaskIndex, Task<TM::Input>>,

    /// Hashmap to store the task responses
    task_responses: HashMap<TaskIndex, HashMap<TaskResponseDigest, TaskResponse<TM::Output>>>,

    /// Avs writer
    task_manager: TM,
}
