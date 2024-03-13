#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
script_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)

CONTRACT_PATH=$script_path/lib
cd $CONTRACT_PATH

forge build



cp $CONTRACT_PATH/out/RegistryCoordinator.sol/RegistryCoordinator.json ../src/RegistryCoordinator.json
cp $CONTRACT_PATH/out/OperatorStateRetriever.sol/OperatorStateRetriever.json ../src/OperatorStateRetriever.json
cp $CONTRACT_PATH/out/StakeRegistry.sol/StakeRegistry.json ../src/StakeRegistry.json
cp $CONTRACT_PATH/out/BLSApkRegistry.sol/BLSApkRegistry.json ../src/BLSApkRegistry.json
cp $CONTRACT_PATH/out/IBLSSignatureChecker.sol/IBLSSignatureChecker.json ../src/IBLSSignatureChecker.json
cp $CONTRACT_PATH/out/ServiceManagerBase.sol/ServiceManagerBase.json ../src/ServiceManagerBase.json
cp $CONTRACT_PATH/out/DelegationManager.sol/DelegationManager.json ../src/DelegationManager.json
cp $CONTRACT_PATH/out/ISlasher.sol/ISlasher.json ../src/ISlasher.json
cp $CONTRACT_PATH/out/StrategyManager.sol/StrategyManager.json ../src/StrategyManager.json
cp $CONTRACT_PATH/out/EigenPod.sol/EigenPod.json ../src/EigenPod.json
cp $CONTRACT_PATH/out/EigenPodManager.sol/EigenPodManager.json ../src/EigenPodManager.json
cp $CONTRACT_PATH/out/IStrategy.sol/IStrategy.json ../src/IStrategy.json
cp $CONTRACT_PATH/out/AVSDirectory.sol/AVSDirectory.json ../src/AVSDirectory.json



