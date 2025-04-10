use bindings::iincrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    NewTaskCreated, TaskResponded,
};
use eigen_challenger::{challenger::ChallengerTaskProcessor, Challenger};

pub mod bindings;

struct ChallengerTaskProcessorImpl {}

impl ChallengerTaskProcessor for ChallengerTaskProcessorImpl {
    type NewTaskEvent = NewTaskCreated;

    type TaskResponseEvent = TaskResponded;

    fn handle_task_creation(&mut self, decoded: alloy::rpc::types::Log<Self::NewTaskEvent>) {
        let data = decoded.data();

        println!("Number to be squared: {:?}", data.task.numberToBeSquared);
        println!("Quorum numbers: {:?}", data.task.quorumNumbers);
        println!(
            "Quorum threshold percentage: {:?}",
            data.task.quorumThresholdPercentage
        );
    }

    fn handle_task_response(&mut self, decoded: alloy::rpc::types::Log<Self::TaskResponseEvent>) {
        let data = decoded.data();

        println!("Task index: {:?}", data.taskResponse.referenceTaskIndex);
        println!("Number squared: {:?}", data.taskResponse.numberSquared);
    }
}

#[tokio::main]
async fn main() {
    let ws_url = "ws://localhost:8545";
    let task_processor = ChallengerTaskProcessorImpl {};
    let mut challenger = Challenger::start_challenger(ws_url, task_processor)
        .await
        .unwrap();
}
