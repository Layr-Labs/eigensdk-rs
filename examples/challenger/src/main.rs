use std::collections::HashMap;

use bindings::iincrediblesquaringtaskmanager::{
    IIncredibleSquaringTaskManager::{
        NewTaskCreated, Task, TaskResponded, TaskResponse, TaskResponseMetadata,
    },
    BN254::G1Point,
};
use eigen_challenger::{challenger::ChallengerTaskProcessor, Challenger};

pub mod bindings;

struct ChallengerTaskProcessorImpl {
    tasks: HashMap<u32, Task>,
    task_responses: HashMap<u32, TaskResponse>,
}

/// 1. Implement ChallengerTaskProcessor trait
impl ChallengerTaskProcessor for ChallengerTaskProcessorImpl {
    type NewTaskEvent = NewTaskCreated;

    type TaskResponseEvent = TaskResponded;

    fn handle_task_creation(&mut self, decoded: alloy::rpc::types::Log<Self::NewTaskEvent>) {
        let data = decoded.data();

        self.tasks.insert(data.taskIndex, data.task.clone());
    }

    fn handle_task_response(&mut self, decoded: alloy::rpc::types::Log<Self::TaskResponseEvent>) {
        let data = decoded.data();
        let task_index = data.taskResponse.referenceTaskIndex;

        self.task_responses
            .insert(task_index, data.taskResponse.clone());

        if self.tasks.contains_key(&task_index) && self.check_task_response(task_index) {
            self.raise_challenge(task_index);
        }
    }
}

impl ChallengerTaskProcessorImpl {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            task_responses: HashMap::new(),
        }
    }

    fn check_task_response(&self, _task_index: u32) -> bool {
        todo!()
    }

    fn raise_challenge(&self, _task_index: u32) {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let ws_url = "ws://localhost:8545";
    let task_processor = ChallengerTaskProcessorImpl::new();
    let mut challenger = Challenger::new(ws_url.to_string(), task_processor);
    challenger.start_challenger().await.unwrap();
}
