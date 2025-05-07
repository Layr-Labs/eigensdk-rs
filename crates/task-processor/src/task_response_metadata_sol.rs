use alloy::sol;

// Metadata of the task response
sol! {
  #[derive(Debug)]
  struct TaskResponseMetadataSol {
      uint32 taskResponsedBlock;
      bytes32 hashOfNonSigners;
  }
}
