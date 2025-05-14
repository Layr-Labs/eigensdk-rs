// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package contractIncredibleDotProductTaskManager

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
	_ = abi.ConvertType
)

// BN254G1Point is an auto generated low-level Go binding around an user-defined struct.
type BN254G1Point struct {
	X *big.Int
	Y *big.Int
}

// BN254G2Point is an auto generated low-level Go binding around an user-defined struct.
type BN254G2Point struct {
	X [2]*big.Int
	Y [2]*big.Int
}

// IBLSSignatureCheckerTypesNonSignerStakesAndSignature is an auto generated low-level Go binding around an user-defined struct.
type IBLSSignatureCheckerTypesNonSignerStakesAndSignature struct {
	NonSignerQuorumBitmapIndices []uint32
	NonSignerPubkeys             []BN254G1Point
	QuorumApks                   []BN254G1Point
	ApkG2                        BN254G2Point
	Sigma                        BN254G1Point
	QuorumApkIndices             []uint32
	TotalStakeIndices            []uint32
	NonSignerStakeIndices        [][]uint32
}

// IBLSSignatureCheckerTypesQuorumStakeTotals is an auto generated low-level Go binding around an user-defined struct.
type IBLSSignatureCheckerTypesQuorumStakeTotals struct {
	SignedStakeForQuorum []*big.Int
	TotalStakeForQuorum  []*big.Int
}

// IIncredibleDotProductTaskManagerDotProductInput is an auto generated low-level Go binding around an user-defined struct.
type IIncredibleDotProductTaskManagerDotProductInput struct {
	X []*big.Int
	Y []*big.Int
}

// IIncredibleDotProductTaskManagerTask is an auto generated low-level Go binding around an user-defined struct.
type IIncredibleDotProductTaskManagerTask struct {
	PointsToMultiply          IIncredibleDotProductTaskManagerDotProductInput
	TaskCreatedBlock          uint32
	QuorumNumbers             []byte
	QuorumThresholdPercentage uint32
}

// IIncredibleDotProductTaskManagerTaskResponse is an auto generated low-level Go binding around an user-defined struct.
type IIncredibleDotProductTaskManagerTaskResponse struct {
	ReferenceTaskIndex uint32
	Result             *big.Int
}

// IIncredibleDotProductTaskManagerTaskResponseMetadata is an auto generated low-level Go binding around an user-defined struct.
type IIncredibleDotProductTaskManagerTaskResponseMetadata struct {
	TaskRespondedBlock uint32
	HashOfNonSigners   [32]byte
}

// OperatorStateRetrieverCheckSignaturesIndices is an auto generated low-level Go binding around an user-defined struct.
type OperatorStateRetrieverCheckSignaturesIndices struct {
	NonSignerQuorumBitmapIndices []uint32
	QuorumApkIndices             []uint32
	TotalStakeIndices            []uint32
	NonSignerStakeIndices        [][]uint32
}

// OperatorStateRetrieverOperator is an auto generated low-level Go binding around an user-defined struct.
type OperatorStateRetrieverOperator struct {
	Operator   common.Address
	OperatorId [32]byte
	Stake      *big.Int
}

// ContractIncredibleDotProductTaskManagerMetaData contains all meta data concerning the ContractIncredibleDotProductTaskManager contract.
var ContractIncredibleDotProductTaskManagerMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"_registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractISlashingRegistryCoordinator\"},{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"_taskResponseWindowBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"TASK_CHALLENGE_WINDOW_BLOCK\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"TASK_RESPONSE_WINDOW_BLOCK\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"WADS_TO_SLASH\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"aggregator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allTaskResponses\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"allocationManager\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"blsApkRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBLSApkRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureCheckerTypes.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureCheckerTypes.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createNewTask\",\"inputs\":[{\"name\":\"points\",\"type\":\"tuple\",\"internalType\":\"structIIncredibleDotProductTaskManager.DotProductInput\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"},{\"name\":\"Y\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}]},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"delegation\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIDelegationManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"generator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getBatchOperatorFromId\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractISlashingRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"operators\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getBatchOperatorId\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractISlashingRegistryCoordinator\"},{\"name\":\"operators\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"outputs\":[{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getCheckSignaturesIndices\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractISlashingRegistryCoordinator\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structOperatorStateRetriever.CheckSignaturesIndices\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractISlashingRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractISlashingRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getQuorumBitmapsAtBlockNumber\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractISlashingRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getTaskResponseWindowBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_aggregator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_generator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_allocationManager\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_slasher\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_serviceManager\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"instantSlasher\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestTaskNum\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"raiseAndResolveChallenge\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIIncredibleDotProductTaskManager.Task\",\"components\":[{\"name\":\"pointsToMultiply\",\"type\":\"tuple\",\"internalType\":\"structIIncredibleDotProductTaskManager.DotProductInput\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"},{\"name\":\"Y\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}]},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIIncredibleDotProductTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"result\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"internalType\":\"structIIncredibleDotProductTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskRespondedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"pubkeysOfNonSigningOperators\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"registryCoordinator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractISlashingRegistryCoordinator\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"respondToTask\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIIncredibleDotProductTaskManager.Task\",\"components\":[{\"name\":\"pointsToMultiply\",\"type\":\"tuple\",\"internalType\":\"structIIncredibleDotProductTaskManager.DotProductInput\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"},{\"name\":\"Y\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}]},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIIncredibleDotProductTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"result\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"nonSignerStakesAndSignature\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureCheckerTypes.NonSignerStakesAndSignature\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerPubkeys\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApks\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"serviceManager\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setStaleStakesForbidden\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stakeRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"staleStakesForbidden\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"taskNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"taskSuccesfullyChallenged\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"NewTaskCreated\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"task\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIIncredibleDotProductTaskManager.Task\",\"components\":[{\"name\":\"pointsToMultiply\",\"type\":\"tuple\",\"internalType\":\"structIIncredibleDotProductTaskManager.DotProductInput\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"},{\"name\":\"Y\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}]},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"StaleStakesForbiddenUpdate\",\"inputs\":[{\"name\":\"value\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TaskChallengedSuccessfully\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"challenger\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TaskChallengedUnsuccessfully\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"challenger\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TaskCompleted\",\"inputs\":[{\"name\":\"taskIndex\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TaskResponded\",\"inputs\":[{\"name\":\"taskResponse\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIIncredibleDotProductTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"result\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"taskResponseMetadata\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structIIncredibleDotProductTaskManager.TaskResponseMetadata\",\"components\":[{\"name\":\"taskRespondedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"hashOfNonSigners\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"BitmapValueTooLarge\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"BytesArrayLengthTooLong\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"BytesArrayNotOrdered\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"CurrentlyPaused\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"ECAddFailed\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"ECMulFailed\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"ExpModFailed\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InputAddressZero\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InputArrayLengthMismatch\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InputEmptyQuorumNumbers\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InputNonSignerLengthMismatch\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InvalidBLSPairingKey\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InvalidBLSSignature\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InvalidNewPausedStatus\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InvalidQuorumApkHash\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InvalidReferenceBlocknumber\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"NonSignerPubkeysNotSorted\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"OnlyPauser\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"OnlyRegistryCoordinatorOwner\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"OnlyUnpauser\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"OperatorNotRegistered\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"ScalarTooLarge\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"StaleStakesForbidden\",\"inputs\":[]}]",
	Bin: "0x61014080604052346101e6576060816152e480380380916100208285610291565b8339810103126101e65780516001600160a01b038116908181036101e65760208301516001600160a01b038116938482036101e657604001519363ffffffff851685036101e657156102825760805260a052604051636830483560e01b8152602081600481855afa9081156101f2575f9161023f575b5060c052604051632efa2ca360e11b815290602090829060049082905afa9081156101f2575f916101fd575b5060e05260c05160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa9081156101f2575f916101ac575b50610100526101205260405161501b90816102c982396080518181816104f301528181610f62015281816116ab0152611e1f015260a051818181610b840152818161127b015281816130cf015281816131b80152818161374a0152613f97015260c051818181611237015281816134ba0152613605015260e0518181816111f3015281816133f60152613eb8015261010051818181611d1b01526132cb0152610120518181816107cf01526118d40152f35b90506020813d6020116101ea575b816101c760209383610291565b810103126101e657516001600160a01b03811681036101e6575f6100fa565b5f80fd5b3d91506101ba565b6040513d5f823e3d90fd5b90506020813d602011610237575b8161021860209383610291565b810103126101e657516001600160a01b03811681036101e6575f6100c2565b3d915061020b565b90506020813d60201161027a575b8161025a60209383610291565b810103126101e657516001600160a01b03811681036101e6576004610096565b3d915061024d565b6339b190bb60e11b5f5260045ffd5b601f909101601f19168101906001600160401b038211908210176102b457604052565b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8063018630671461029a578063136439dd14610295578063171f1d5b146102905780631ad43189146101e6578063245a7bfc1461028b5780632cb223d5146102865780632d89f6fc1461028157806331b36bd91461027c5780633563b0d1146102775780633998fdd314610272578063416c7e5e1461026d5780634d2b57fe146102685780634f739f7414610263578063595c6a671461025e5780635a2d7f02146102595780635ac86ab7146102545780635c1556621461024f5780635c975abb1461024a5780635decc3f5146102455780635df4594614610240578063683048351461023b5780636d14a987146102365780636efb463614610231578063715018a61461022c57806372d18e8d1461021d5780637afa1eed14610227578063886f1195146102225780638b00ce7c1461021d5780638da5cb5b146102185780639b290e9814610213578063b490bb411461020e578063b98d090814610209578063c9b1479714610204578063ca8aa7c7146101ff578063cc2a9a5b146101fa578063cefdc1d4146101f5578063df5cf723146101f0578063f2fde38b146101eb578063f5c9899d146101e6578063f63c5bab146101e15763fabc1cbc146101dc575f80fd5b611df6565b611ddb565b6107b3565b611d4a565b611d06565b611bc2565b611a82565b611a5a565b6117cb565b6117a9565b611738565b611702565b6116da565b61164b565b611696565b61166e565b6115f0565b611543565b611266565b611222565b6111de565b6111a0565b611183565b61100a565b610fd7565b610faa565b610f37565b610e90565b610cb2565b610b52565b610b20565b610aa6565b6108fc565b610854565b61081b565b6107f3565b610741565b6104c3565b61030a565b60409060231901126102b057602490565b5f80fd5b908160409103126102b05790565b63ffffffff8116036102b057565b35906102db826102c2565b565b9181601f840112156102b0578235916001600160401b0383116102b057602083818601950101116102b057565b346102b05760603660031901126102b0576004356001600160401b0381116102b05761033a9036906004016102b4565b60243590610347826102c2565b6044356001600160401b0381116102b0576103669036906004016102dd565b60ce5491939092916001600160a01b03163303610474576104729361045c936103ba6103c19361039f610397611ef1565b963690611f1c565b86524363ffffffff16602087015263ffffffff166060860152565b36916109c9565b604082015260405160208101906103ea816103dc8585611fa5565b03601f1981018352826105d0565b5190206104136103ff60c95463ffffffff1690565b63ffffffff165f5260ca60205260405f2090565b5560c95463ffffffff16907ff3d0298607cbbde38baba3c3915d277f1ffd80a384095a02a4a4d7c082ae6cd96040518061045463ffffffff86169482611fa5565b0390a261202b565b63ffffffff1663ffffffff1960c954161760c955565b005b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b346102b05760203660031901126102b05760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561057c5761047292610539915f9161054d575b5061209a565b610548606654828116146120b0565b614774565b61056f915060203d602011610575575b61056781836105d0565b81019061207a565b5f610533565b503d61055d565b61208f565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176105b057604052565b610581565b608081019081106001600160401b038211176105b057604052565b90601f801991011681019081106001600160401b038211176105b057604052565b604051906102db610100836105d0565b604051906102db6040836105d0565b604051906102db6060836105d0565b604051906102db60a0836105d0565b906102db60405192836105d0565b60409060e31901126102b0576040519061065582610595565b60e4358252610104356020830152565b91908260409103126102b05760405161067d81610595565b6020808294803584520135910152565b9080601f830112156102b057604051916106a86040846105d0565b8290604081019283116102b057905b8282106106c45750505090565b81358152602091820191016106b7565b9060806063198301126102b0576040516106ed81610595565b602061070882946106ff81606461068d565b845260a461068d565b910152565b91906080838203126102b05760206107086040519261072b84610595565b60408496610739838261068d565b86520161068d565b346102b0576101203660031901126102b05760043560403660231901126102b057610799604091825161077381610595565b60243581526044356020820152610789366106d4565b906107933661063c565b92612104565b8251911515825215156020820152f35b5f9103126102b057565b346102b0575f3660031901126102b057602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346102b0575f3660031901126102b05760cd546040516001600160a01b039091168152602090f35b346102b05760203660031901126102b05763ffffffff60043561083d816102c2565b165f5260cb602052602060405f2054604051908152f35b346102b05760203660031901126102b05763ffffffff600435610876816102c2565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b038116036102b057565b6001600160401b0381116105b05760051b60200190565b90602080835192838152019201905f5b8181106108d25750505090565b82518452602093840193909201916001016108c5565b9060206108f99281815201906108b5565b90565b346102b05760403660031901126102b0576004356109198161088d565b602435906001600160401b0382116102b057366023830112156102b0578160040135916109458361089e565b9261095360405194856105d0565b8084526024602085019160051b830101913683116102b057602401905b82821061099457610990610984868661226e565b604051918291826108e8565b0390f35b6020809183356109a38161088d565b815201910190610970565b6001600160401b0381116105b057601f01601f191660200190565b9291926109d5826109ae565b916109e360405193846105d0565b8294818452818301116102b0578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b838310610a2a57505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b808210610a6a5750505060208060019297019301930191939290610a1b565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610a4b565b346102b05760603660031901126102b057600435610ac38161088d565b6024356001600160401b0381116102b057366023820112156102b05761099091610afa610b0c9236906024816004013591016109c9565b60443591610b07836102c2565b6124ac565b6040519182916020835260208301906109ff565b346102b0575f3660031901126102b05760d1546040516001600160a01b039091168152602090f35b801515036102b057565b346102b05760203660031901126102b057600435610b6f81610b48565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c575f91610be4575b506001600160a01b03163303610bd55761047290614c07565b637070f3b160e11b5f5260045ffd5b610c06915060203d602011610c0c575b610bfe81836105d0565b810190612333565b5f610bbc565b503d610bf4565b9080601f830112156102b0578135610c2a8161089e565b92610c3860405194856105d0565b81845260208085019260051b8201019283116102b057602001905b828210610c605750505090565b8135815260209182019101610c53565b60206040818301928281528451809452019201905f5b818110610c935750505090565b82516001600160a01b0316845260209384019390920191600101610c86565b346102b05760403660031901126102b057600435610ccf8161088d565b6024356001600160401b0381116102b057610cee903690600401610c13565b610cf8815161220c565b916001600160a01b03165f5b8251811015610d9557806020610d1d610d3d938661224b565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561057c57600192610d71915f91610d77575b50610d62838861224b565b6001600160a01b039091169052565b01610d04565b610d8f915060203d8111610c0c57610bfe81836105d0565b5f610d57565b604051806109908682610c70565b90602080835192838152019201905f5b818110610dc05750505090565b825163ffffffff16845260209384019390920191600101610db3565b90602082526060610e2a610e15610dff84516080602088015260a0870190610da3565b6020850151868203601f19016040880152610da3565b6040840151858203601f190184870152610da3565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610e6357505050505090565b9091929394602080610e81600193601f198682030187528951610da3565b97019301930191939290610e54565b346102b05760803660031901126102b057600435610ead8161088d565b60243590610eba826102c2565b6044356001600160401b0381116102b057610ed99036906004016102dd565b91606435926001600160401b0384116102b057366023850112156102b0578360040135926001600160401b0384116102b0573660248560051b870101116102b057610990956024610f2b9601936129b1565b60405191829182610ddc565b346102b0575f3660031901126102b05760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561057c57610fa2915f9161054d575061209a565b610472614740565b346102b0575f3660031901126102b057602060405167016345785d8a00008152f35b60ff8116036102b057565b346102b05760203660031901126102b0576020600160ff600435610ffa81610fcc565b161b806066541614604051908152f35b346102b05760603660031901126102b0576004356110278161088d565b6024356001600160401b0381116102b057611046903690600401610c13565b60443591611053836102c2565b6040516361c8a12f60e11b8152906001600160a01b03165f828061107b868860048401612e19565b0381845afa91821561057c575f9261115f575b50611099835161220c565b935f5b8451811015611151576110af818661224b565b51906020836110cb6110c1848961224b565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa801561057c576001925f91611123575b50828060c01b031661111c828961224b565b520161109c565b611144915060203d811161114a575b61113c81836105d0565b810190612928565b5f61110a565b503d611132565b6040518061099088826108e8565b61117c9192503d805f833e61117481836105d0565b810190612806565b905f61108e565b346102b0575f3660031901126102b0576020606654604051908152f35b346102b05760203660031901126102b05763ffffffff6004356111c2816102c2565b165f5260cc602052602060ff60405f2054166040519015158152f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080601f830112156102b05781356112c18161089e565b926112cf60405194856105d0565b81845260208085019260051b8201019283116102b057602001905b8282106112f75750505090565b602080918335611306816102c2565b8152019101906112ea565b81601f820112156102b05780356113278161089e565b9261133560405194856105d0565b81845260208085019260061b840101928184116102b057602001915b83831061135f575050505090565b602060409161136e8486610665565b815201920191611351565b9080601f830112156102b05781356113908161089e565b9261139e60405194856105d0565b81845260208085019260051b820101918383116102b05760208201905b8382106113ca57505050505090565b81356001600160401b0381116102b0576020916113ec878480948801016112aa565b8152019101906113bb565b919091610180818403126102b05761140d6105f1565b9281356001600160401b0381116102b0578161142a9184016112aa565b845260208201356001600160401b0381116102b0578161144b918401611311565b602085015260408201356001600160401b0381116102b0578161146f918401611311565b6040850152611481816060840161070d565b60608501526114938160e08401610665565b60808501526101208201356001600160401b0381116102b057816114b89184016112aa565b60a08501526101408201356001600160401b0381116102b057816114dd9184016112aa565b60c08501526101608201356001600160401b0381116102b0576115009201611379565b60e0830152565b90602080835192838152019201905f5b8181106115245750505090565b82516001600160601b0316845260209384019390920191600101611517565b346102b05760803660031901126102b0576004356024356001600160401b0381116102b0576115769036906004016102dd565b9091604435611584816102c2565b606435926001600160401b0384116102b0576115e6946115ab6115b19536906004016113f7565b93612ff2565b6040519283926040845260206115d282516040808801526080870190611507565b910151848203603f19016060860152611507565b9060208301520390f35b346102b0575f3660031901126102b057611608614de3565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346102b0575f3660031901126102b057602063ffffffff60c95416604051908152f35b346102b0575f3660031901126102b05760ce546040516001600160a01b039091168152602090f35b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b0575f3660031901126102b0576033546040516001600160a01b039091168152602090f35b346102b0575f3660031901126102b05760cf546040516001600160a01b039091168152602090f35b908160809103126102b05790565b346102b05760c03660031901126102b0576004356001600160401b0381116102b05761176890369060040161172a565b6117713661029f565b9060403660631901126102b05760a4356001600160401b0381116102b057610472926117a36064923690600401611311565b92613d0a565b346102b0575f3660031901126102b057602060ff609754166040519015158152f35b346102b05760803660031901126102b0576004356001600160401b0381116102b0576117fb90369060040161172a565b6118043661029f565b906064356001600160401b0381116102b0576118249036906004016113f7565b60cd549092906001600160a01b03163303611a155761184760208394930161388f565b916119286118586040860186613b81565b92909461189661186a6060890161388f565b97604051611880816103dc6020820194856143b9565b51902061188f6103ff8861388f565b5414614471565b6118c06118b96118a58761388f565b63ffffffff165f5260cb60205260405f2090565b54156144e3565b8363ffffffff43169661190a6119026118f97f000000000000000000000000000000000000000000000000000000000000000086612060565b63ffffffff1690565b891115614544565b6040516020810190611920816103dc8b856145a6565b519020612ff2565b919060ff5f9616955b8281106119b3577f349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a868686611973611967610601565b63ffffffff9094168452565b6020830152604051602081019061198f816103dc86868661464a565b51902061199e6118a58361388f565b556119ae6040519283928361464a565b0390a1005b80611a0f6119eb6119e66119da6119cd600196885161224b565b516001600160601b031690565b6001600160601b031690565b613ab7565b611a086119da8b611a036119cd8760208b015161224b565b6145b6565b11156145d9565b01611931565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b346102b0575f3660031901126102b05760d0546040516001600160a01b039091168152602090f35b346102b05760c03660031901126102b057600435611a9f8161088d565b611b1f602435611aae8161088d565b604435611aba8161088d565b606435611ac68161088d565b60843591611ad38361088d565b60a43593611ae08561088d565b5f5496611b0560ff60088a901c16158099819a611b9d575b8115611b7d575b50614674565b87611b16600160ff195f5416175f55565b611b66576146d7565b611b2557005b611b3361ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016119ae565b611b7861010061ff00195f5416175f55565b6146d7565b303b15915081611b8f575b505f611aff565b60ff1660011490505f611b88565b600160ff8216109150611af8565b6040906108f99392815281602082015201906109ff565b346102b05760603660031901126102b057600435611bdf8161088d565b602435604435611bee816102c2565b611c2f611bf96121ea565b9280611c048561223e565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401612e19565b0381875afa93841561057c5783611c596118f96110c1611c8e986020975f91611cec575b5061223e565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561057c57611cbd925f91611ccd575b506001600160c01b031692611cb784614e83565b906124ac565b9061099060405192839283611bab565b611ce6915060203d60201161114a5761113c81836105d0565b5f611ca3565b611d0091503d805f833e61117481836105d0565b5f611c53565b346102b0575f3660031901126102b0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346102b05760203660031901126102b057600435611d678161088d565b611d6f614de3565b6001600160a01b03811615611d875761047290614e3b565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346102b0575f3660031901126102b057602060405160648152f35b346102b05760203660031901126102b05760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c575f91611eb9575b506001600160a01b03163303611eaa57611e786066541982198116146120b0565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b611ed2915060203d602011610c0c57610bfe81836105d0565b5f611e57565b60405190611ee582610595565b60606020838281520152565b60405190611efe826105b5565b5f606083611f0a611ed8565b81528260208201528160408201520152565b91906040838203126102b05760405190611f3582610595565b819380356001600160401b0381116102b05782611f53918301610c13565b83526020810135916001600160401b0383116102b0576020926107089201610c13565b6108f9903690611f1c565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b60208152608063ffffffff606061200d611fec86518560208801526020611fd88251604060a08b015260e08a01906108b5565b910151878203609f190160c08901526108b5565b8360208801511660408701526040870151601f198783030184880152611f81565b9401511691015290565b634e487b7160e01b5f52601160045260245ffd5b63ffffffff60019116019063ffffffff821161204357565b612017565b63ffffffff60649116019063ffffffff821161204357565b9063ffffffff8091169116019063ffffffff821161204357565b908160209103126102b057516108f981610b48565b6040513d5f823e3d90fd5b156120a157565b631d77d47760e21b5f5260045ffd5b156120b757565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b9060028110156120eb5760051b0190565b6120c6565b634e487b7160e01b5f52601260045260245ffd5b6121e06121bd6121e6956121b76121b085875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e084015261010083015261218781610120840103601f1981018352826105d0565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b80966147ea565b90614830565b926121b76121d26121cc614892565b94614989565b916121db614aa5565b6147ea565b91614ad9565b9091565b604080519091906121fb83826105d0565b6001815291601f1901366020840137565b906122168261089e565b61222360405191826105d0565b8281528092612234601f199161089e565b0190602036910137565b8051156120eb5760200190565b80518210156120eb5760209160051b010190565b908160209103126102b0575190565b91909161227b835161220c565b925f5b815181101561232e578060206122a761229a6122d0948661224b565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa801561057c576001925f91612300575b506122f9828861224b565b520161227e565b612321915060203d8111612327575b61231981836105d0565b81019061225f565b5f6122ee565b503d61230f565b505050565b908160209103126102b057516108f98161088d565b906123528261089e565b61235f60405191826105d0565b82815260208193612372601f199161089e565b0191015f5b82811061238357505050565b606082820152602001612377565b9081518110156120eb570160200190565b6020818303126102b0578051906001600160401b0382116102b057019080601f830112156102b05781516123d58161089e565b926123e360405194856105d0565b81845260208085019260051b8201019283116102b057602001905b82821061240b5750505090565b81518152602091820191016123fe565b906124258261089e565b61243260405191826105d0565b8281528092612443601f199161089e565b015f5b81811061245257505050565b6040519060608201918083106001600160401b038411176105b0576020926040525f81525f838201525f604082015282828601015201612446565b908160209103126102b057516001600160601b03811681036102b05790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa94851561057c575f956127c1575b50604051634f4c91e160e11b815294602086600481855afa91821561057c576004965f9361279f575b5060209060405197888092632efa2ca360e11b82525afa95861561057c575f9661277e575b5061253a8593929551612348565b945f935b80518510156127745761256b6125656125578784612391565b516001600160f81b03191690565b60f81c90565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa93841561057c575f94612750575b506125bb845161241b565b6125c5888b61224b565b526125d0878a61224b565b505f5b845181101561273f578060206125ec61260e938861224b565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa91821561057c575f9261271f575b50612634818761224b565b518a60208a612643858b61224b565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa93841561057c576126d68c8f6126d16001986126e89789975f926126ef575b506126bc6126ad610610565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b61224b565b51906126e2838361224b565b5261224b565b50016125d3565b61271191925060203d8111612718575b61270981836105d0565b81019061248d565b905f6126a1565b503d6126ff565b61273891925060203d8111610c0c57610bfe81836105d0565b905f612629565b50600190960195909450915061253e565b61276d9194503d805f833e61276581836105d0565b8101906123a2565b925f6125b0565b5050509350505090565b61279891965060203d602011610c0c57610bfe81836105d0565b945f61252c565b60209193506127ba90823d8411610c0c57610bfe81836105d0565b9290612507565b6127db91955060203d602011610c0c57610bfe81836105d0565b935f6124de565b604051906127ef826105b5565b606080838181528160208201528160408201520152565b6020818303126102b0578051906001600160401b0382116102b057019080601f830112156102b05781516128398161089e565b9261284760405194856105d0565b81845260208085019260051b8201019283116102b057602001905b82821061286f5750505090565b60208091835161287e816102c2565b815201910190612862565b81835290916001600160fb1b0383116102b05760209260051b809284830137010190565b60409063ffffffff6108f995931681528160208201520191612889565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6108f9959316815281602082015201916128ca565b60ff1660ff81146120435760010190565b91908110156120eb5760051b0190565b908160209103126102b057516001600160c01b03811681036102b05790565b1561294e57565b6325ec6c1f60e01b5f5260045ffd5b908210156120eb570190565b908160209103126102b057516108f9816102c2565b5f1981146120435760010190565b916129aa60209263ffffffff929695966040865260408601916128ca565b9416910152565b95939495929091926129c16127e2565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa94851561057c575f95612df8575b506129fe6127e2565b946040516361c8a12f60e11b81525f8180612a1e8d8d8b600485016128ad565b0381885afa90811561057c575f91612dde575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f8180612a6185878b600485016128ea565b0381875afa90811561057c575f91612dc4575b506040870152612a8381612348565b9860608701998a525f5b60ff811683811015612d0f57885f612ab6838f612aa98861220c565b9051906126e2838361224b565b505f8a868f5b818410612b39575050505090508c612ad38261220c565b915f5b818110612b0057505091612af591612afb949351906126e2838361224b565b50612907565b612a8d565b80612b33612b1e6110c1600194612b188a895161224b565b5161224b565b612b28838861224b565b9063ffffffff169052565b01612ad6565b6110c184612b4e8160209695612b5695612918565b35975161224b565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561057c57888f888a918f94612bfb6001612bee81938d809d5f92612ce3575b50612565612bca612bd892612bc3878060c01b0386161515612947565b8b8d61295d565b356001600160f81b03191690565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14612c17575b5050505050600191925001908a918a868f612abc565b8597612c3993612c32602097999861256595612bca95612918565b359561295d565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa90811561057c578f612c9790612c9c9383886001975f93612cab575b50612b1890612b2893945161224b565b61297e565b905082918a888f888a91612c01565b612b28935090612cd4612b189260203d8111612cdc575b612ccc81836105d0565b810190612969565b935090612c87565b503d612cc2565b612bd8919250612bca612d066125659260203d811161114a5761113c81836105d0565b93925050612ba6565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa90811561057c57612d65945f948593612da3575b5060405163354952a360e21b8152958694859384936004850161298c565b03916001600160a01b03165afa90811561057c575f91612d89575b50602082015290565b612d9d91503d805f833e61117481836105d0565b5f612d80565b612dbd91935060203d602011610c0c57610bfe81836105d0565b915f612d47565b612dd891503d805f833e61117481836105d0565b5f612a74565b612df291503d805f833e61117481836105d0565b5f612a31565b612e1291955060203d602011610c0c57610bfe81836105d0565b935f6129f5565b60409063ffffffff6108f9949316815281602082015201906108b5565b15612e3d57565b62f8202d60e51b5f5260045ffd5b15612e5257565b6343714afd60e01b5f5260045ffd5b15612e6857565b635f832f4160e01b5f5260045ffd5b15612e7e57565b634b874f4560e01b5f5260045ffd5b908160209103126102b057516108f981610fcc565b5f1981019190821161204357565b15612eb757565b633fdc650560e21b5f5260045ffd5b906001820180921161204357565b906002820180921161204357565b906003820180921161204357565b906004820180921161204357565b906005820180921161204357565b9190820180921161204357565b15612f2057565b63affc5edb60e01b5f5260045ffd5b908160209103126102b0575167ffffffffffffffff19811681036102b05790565b15612f5757565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b03821161204357565b15612f8d57565b6367988d3360e01b5f5260045ffd5b15612fa357565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b818110612fdc5750505090565b8251845260209384019390920191600101612fcf565b949392909193613000611ed8565b5061300c851515612e36565b604084015151851480613881575b80613873575b80613865575b61302f90612e4b565b61304160208501515185515114612e61565b61305863ffffffff431663ffffffff841610612e77565b613060610601565b5f81525f602082015292613072611ed8565b61307b8761220c565b60208201526130898761220c565b8152613093611ed8565b926130a260208801515161220c565b84526130b260208801515161220c565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561057c5761311b915f91613836575b50613116368b876109c9565b614c45565b985f965b6020890151805189101561328d576020886131826110c18c61317a8f96868e61315f61314c86809561224b565b5180515f526020015160205260405f2090565b61316c848484015161224b565b528261325a575b015161224b565b51955161224b565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa91821561057c576121b78a61322f8f6132288f8460208f9261321f936132178460019e6132359e5f9161323d575b508f8060c01b0316925161224b565b52015161224b565b51938d5161224b565b5116614c70565b90614ca1565b97019661311f565b6132549150863d811161114a5761113c81836105d0565b5f613208565b61328861326a848484015161224b565b516132818484015161327b87612ea2565b9061224b565b5110612eb0565b613173565b509095979496506132a2919893929950614d5e565b916132af60975460ff1690565b90811561382e576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c575f9161380f575b5091905b5f925b8184106133605750505050509261334761334261333b61335a95856103dc9860806060602099015192015192612104565b9190612f86565b612f9c565b0151604051928391602083019586612fb2565b51902090565b92989596909399919794878b888c888d613709575b6110c18260a06133b5612565612bca846133bd976133af6133a161314c8f9c604060209f9e015161224b565b67ffffffffffffffff191690565b9b61295d565b97015161224b565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561057c576134816110c18f958f906134798f978f96848f61347360c09661346c848f60209f90613173612bca996040936125659c5f916136db575b5067ffffffffffffffff19918216911614612f50565b5190614830565b9c61295d565b96015161224b565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561057c5761350e918c8f925f926136b7575b5060206135009293015161224b565b906001600160601b03169052565b61352e8c6135008c6135276119cd82602086015161224b565b925161224b565b5f985f5b60208a01515181101561369e578b8d61357089613563612565612bca868f8961355b915161224b565b51948761295d565b60ff161c60019081161490565b61357f575b5050600101613532565b8a8a613601859f948f9686612b188f9360e06135b86110c19560206135b0612565612bca839f6135c19c899161295d565b9a015161224b565b519b015161224b565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561057c578f61366d908f936001959486955f92613678575b50613667613500929351936136626119cd848761224b565b612f66565b9261224b565b019a90508b8d613575565b61350092506136976136679160203d81116127185761270981836105d0565b925061364a565b5093919796996001919699509a94929a0192919061330a565b61350092506136d4602091823d81116127185761270981836105d0565b92506134f1565b60206136fc92503d8111613702575b6136f481836105d0565b810190612f2f565b5f613456565b503d6136ea565b6137469450613723925061256591612bca9160209561295d565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561057c576020896133bd8f938f60a08f97612565612bca8f8f906133af6133a161314c8f60408b96918f88936110c19f6137ca906137d0936133b59f5f926137e6575b5063ffffffff809116931690612f0c565b11612f19565b5050505050509750505050505092935050613375565b602063ffffffff9293508291613807913d81116123275761231981836105d0565b9291506137b9565b613828915060203d602011612cdc57612ccc81836105d0565b5f613303565b5f9190613307565b613858915060203d60201161385e575b61385081836105d0565b810190612e8d565b5f61310a565b503d613846565b5060e0840151518514613026565b5060c0840151518514613020565b5060a084015151851461301a565b356108f9816102c2565b903590603e19813603018212156102b0570190565b156138b557565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b6020809163ffffffff8135613918816102c2565b1684520135910152565b9092916020606091613938846080810197613904565b63ffffffff8135613948816102c2565b1660408501520135910152565b1561395c57565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156139ce57565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b15613a4c57565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b9060648202918083046064149015171561204357565b9060068202918083046006149015171561204357565b8181029291811591840414171561204357565b15613afd57565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b903590601e19813603018212156102b057018035906001600160401b0382116102b0576020019181360383136102b057565b6020818303126102b0578051906001600160401b0382116102b057019080601f830112156102b0578151613be68161089e565b92613bf460405194856105d0565b81845260208085019260051b8201019283116102b057602001905b828210613c1c5750505090565b602080918351613c2b8161088d565b815201910190613c0f565b60405190613c456040836105d0565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b818110613ceb575050506080613cd66108f994956060850151601f1985830301848601526108b5565b9201519060a0601f1982850301910152611f81565b82516001600160a01b0316875260209687019690920191600101613cad565b9193929093613d188561388f565b93613d2b613d268580613899565b611f76565b94613d4f613d478263ffffffff165f5260cb60205260405f2090565b5415156138ae565b613d8b613d6a8263ffffffff165f5260cb60205260405f2090565b5488604051613d82816103dc89602083019586613922565b51902014613955565b613db6613db0613da98363ffffffff165f5260cc60205260405f2090565b5460ff1690565b156139c7565b613ddb613dcd6118f9613dc88661388f565b612048565b63ffffffff43161115613a45565b5f935f9560208801955b88518051891015613e2457600191613e16613e038b613e1c9461224b565b51613e0f8c8c5161224b565b5190613ae3565b90612f0c565b970196613de5565b50929750945094506020600192970135141461432357613e44835161220c565b935f5b8451811015613e715780613e6061314c6001938861224b565b613e6a828961224b565b5201613e47565b509092939194613eab60208701946020613e8a8761388f565b604051613e9f816103dc8a8683019586612fb2565b51902091013514613af6565b613eb5855161220c565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b8751811015613f6557806020613efc613f1c938961224b565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa91821561057c57600192613f41915f91613f47575b50610d62838d61224b565b01613ee3565b613f5f915060203d8111610c0c57610bfe81836105d0565b5f613f36565b5092969550925092613fc4613f8d613f956040860194613f858688613b81565b93909161388f565b9236916109c9565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166124ac565b945f915b86518310156142c5575f979697955b613fe1848a61224b565b51518710156142b55797614022986020806140008a612b18898761224b565b510151604051809c81926308f6629d60e31b8352600483019190602083019252565b0381865afa998a1561057c575f9a614295575b506001985f5b85518110156142875761406061405461229a838961224b565b6001600160a01b031690565b6001600160a01b038d16146140775760010161403b565b5099909791985060015f5b151514614097575b5060010195979697613fd7565b98969786985f879593986140fc60ff6140d4612565612bca8c6140ce6141519f6140c860d15460018060a01b031690565b98613b81565b9061295d565b6140ee6140df610601565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d05461411390614054906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529098899190829081906044820190565b03915afa96871561057c575f97614263575b5061416e875161220c565b985f5b8a51811015614197578067016345785d8a00006141906001938e61224b565b5201614171565b509a9294966141d560ff6141bc612565612bca8f9d979f969e966140ce8e8e92613b81565b6141c76126ad61061f565b1663ffffffff166020860152565b604084015260608301526141e7613c36565b608083015260cf5461420390614054906001600160a01b031681565b803b156102b057604051636a669b4160e01b8152925f91849182908490829061422f9060048301613c66565b03925af191821561057c57600192614249575b509061408a565b806142575f61425d936105d0565b806107a9565b5f614242565b6142809197503d805f833e61427881836105d0565b810190613bb3565b955f614163565b509990979198600190614082565b6142ae919a5060203d8111610c0c57610bfe81836105d0565b985f614035565b9697969550600190920191613fc8565b50505050509190506142f56142e88263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b50505063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b9035601e19823603018112156102b05701602081359101916001600160401b0382116102b0578160051b360383136102b057565b9035601e19823603018112156102b05701602081359101916001600160401b0382116102b05781360383136102b057565b60208152813590603e19833603018212156102b0576080614466606061445f614425876108f997018560208801526144136144086143f78380614354565b604060a08c015260e08b0191612889565b916020810190614354565b888303609f190160c08a015290612889565b61444161443460208a016102d0565b63ffffffff166040880152565b61444e6040890189614388565b878303601f190185890152906128ca565b95016102d0565b63ffffffff16910152565b1561447857565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156144ea57565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b1561454b57565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6040810192916102db9190613904565b906001600160601b03809116911602906001600160601b03821691820361204357565b156145e057565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091614660846080810197613904565b63ffffffff81511660408501520151910152565b1561467b57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6146e090614e3b565b60cd80546001600160a01b03199081166001600160a01b039384161790915560ce805482169383169390931790925560d0805483169382169390931790925560cf805482169383169390931790925560d180549092169216919091179055565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b604051906147b382610595565b5f6020838281520152565b604051906101806147cf81846105d0565b368337565b604051906147e36020836105d0565b6020368337565b919060409060606147f96147a6565b948592602085519261480b85856105d0565b8436853780518452015160208301528482015260076107cf195a01fa1561482e57565bfe5b6020929160806040926148416147a6565b9586938186519361485286866105d0565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa801561482e571561488357565b63d4b68fd760e01b5f5260045ffd5b60405161489e81610595565b60409081516148ad83826105d0565b82368237815260208251916148c284846105d0565b83368437015280516148d482826105d0565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061492a83836105d0565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d602083015261497f835193846105d0565b8252602082015290565b5f516020614fc65f395f51905f52906149a06147a6565b505f919006602060c0835b614aa0575f935f516020614fc65f395f51905f52600381868181800909086040516149d685826105d0565b843682378481856040516149ea82826105d0565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614fc65f395f51905f5260a082015260056107cf195a01fa801561482e57614a5490614faf565b5191614aa0575f516020614fc65f395f51905f5282800914614a8b57505f516020614fc65f395f51905f5260015f940892936149ab565b92935050614a97610601565b92835282015290565b6120f0565b614aad6147a6565b50604051614aba81610595565b600181526002602082015290565b90600c8110156120eb5760051b0190565b93929091614ae7604061062e565b9485526020850152614af9604061062e565b9182526020820152614b096147be565b925f5b60028110614b3657505050602061018092614b256147d4565b93849160086201d4c0fa9151151590565b80614b42600192613acd565b614b4c82856120da565b5151614b588289614ac8565b526020614b6583866120da565b510151614b7a614b7483612ec6565b89614ac8565b52614b8582866120da565b515151614b94614b7483612ed4565b52614baa614ba283876120da565b515160200190565b51614bb7614b7483612ee2565b526020614bc483876120da565b51015151614bd4614b7483612ef0565b52614c00614bfa614bf36020614bea868a6120da565b51015160200190565b5192612efe565b88614ac8565b5201614b0c565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614c5360ff93614f37565b928392161b1115614c615790565b63ca95733360e01b5f5260045ffd5b805f915b614c7c575090565b5f1981018181116120435761ffff9116911661ffff8114612043576001019080614c74565b90614caa6147a6565b5061ffff811690610200821015614d4f5760018214614d4a57614ccb610601565b5f81525f602082015292906001905f925b61ffff8316851015614cf057505050505090565b600161ffff831660ff86161c811614614d2a575b6001614d20614d158360ff94614830565b9460011b61fffe1690565b9401169291614cdc565b946001614d20614d15614d3f8960ff95614830565b989350505050614d04565b505090565b637fc4ea7d60e11b5f5260045ffd5b614d666147a6565b50805190811580614dd7575b15614d93575050604051614d876040826105d0565b5f81525f602082015290565b60205f516020614fc65f395f51905f52910151065f516020614fc65f395f51905f52035f516020614fc65f395f51905f528111612043576040519161497f83610595565b50602081015115614d72565b6033546001600160a01b03163303614df757565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff614e8f82614c70565b16614e99816109ae565b90614ea760405192836105d0565b808252614eb6601f19916109ae565b013660208301375f5f5b8251821080614f16575b15614f0f576001811b8416614ee8575b614ee39061297e565b614ec0565b906001614ee39160ff60f81b8460f81b165f1a614f058287612391565b5301919050614eda565b5050905090565b506101008110614eca565b15614f2857565b631019106960e31b5f5260045ffd5b90610100825111614fa057815115614f9b57602082015160019060f81c81901b5b8351821015614f9657600190614f81614f776125656125578689612391565b60ff600191161b90565b90614f8d818311614f21565b17910190614f58565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b15614fb657565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a264697066735822122010741e868abe01c7431a6f0f3ad7c8e5a5631193f6867a4f74abd52b6e427ad564736f6c634300081b0033",
}

// ContractIncredibleDotProductTaskManagerABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractIncredibleDotProductTaskManagerMetaData.ABI instead.
var ContractIncredibleDotProductTaskManagerABI = ContractIncredibleDotProductTaskManagerMetaData.ABI

// ContractIncredibleDotProductTaskManagerBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractIncredibleDotProductTaskManagerMetaData.Bin instead.
var ContractIncredibleDotProductTaskManagerBin = ContractIncredibleDotProductTaskManagerMetaData.Bin

// DeployContractIncredibleDotProductTaskManager deploys a new Ethereum contract, binding an instance of ContractIncredibleDotProductTaskManager to it.
func DeployContractIncredibleDotProductTaskManager(auth *bind.TransactOpts, backend bind.ContractBackend, _registryCoordinator common.Address, _pauserRegistry common.Address, _taskResponseWindowBlock uint32) (common.Address, *types.Transaction, *ContractIncredibleDotProductTaskManager, error) {
	parsed, err := ContractIncredibleDotProductTaskManagerMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractIncredibleDotProductTaskManagerBin), backend, _registryCoordinator, _pauserRegistry, _taskResponseWindowBlock)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &ContractIncredibleDotProductTaskManager{ContractIncredibleDotProductTaskManagerCaller: ContractIncredibleDotProductTaskManagerCaller{contract: contract}, ContractIncredibleDotProductTaskManagerTransactor: ContractIncredibleDotProductTaskManagerTransactor{contract: contract}, ContractIncredibleDotProductTaskManagerFilterer: ContractIncredibleDotProductTaskManagerFilterer{contract: contract}}, nil
}

// ContractIncredibleDotProductTaskManager is an auto generated Go binding around an Ethereum contract.
type ContractIncredibleDotProductTaskManager struct {
	ContractIncredibleDotProductTaskManagerCaller     // Read-only binding to the contract
	ContractIncredibleDotProductTaskManagerTransactor // Write-only binding to the contract
	ContractIncredibleDotProductTaskManagerFilterer   // Log filterer for contract events
}

// ContractIncredibleDotProductTaskManagerCaller is an auto generated read-only Go binding around an Ethereum contract.
type ContractIncredibleDotProductTaskManagerCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractIncredibleDotProductTaskManagerTransactor is an auto generated write-only Go binding around an Ethereum contract.
type ContractIncredibleDotProductTaskManagerTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractIncredibleDotProductTaskManagerFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type ContractIncredibleDotProductTaskManagerFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractIncredibleDotProductTaskManagerSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type ContractIncredibleDotProductTaskManagerSession struct {
	Contract     *ContractIncredibleDotProductTaskManager // Generic contract binding to set the session for
	CallOpts     bind.CallOpts                            // Call options to use throughout this session
	TransactOpts bind.TransactOpts                        // Transaction auth options to use throughout this session
}

// ContractIncredibleDotProductTaskManagerCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type ContractIncredibleDotProductTaskManagerCallerSession struct {
	Contract *ContractIncredibleDotProductTaskManagerCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                                  // Call options to use throughout this session
}

// ContractIncredibleDotProductTaskManagerTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type ContractIncredibleDotProductTaskManagerTransactorSession struct {
	Contract     *ContractIncredibleDotProductTaskManagerTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                                  // Transaction auth options to use throughout this session
}

// ContractIncredibleDotProductTaskManagerRaw is an auto generated low-level Go binding around an Ethereum contract.
type ContractIncredibleDotProductTaskManagerRaw struct {
	Contract *ContractIncredibleDotProductTaskManager // Generic contract binding to access the raw methods on
}

// ContractIncredibleDotProductTaskManagerCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type ContractIncredibleDotProductTaskManagerCallerRaw struct {
	Contract *ContractIncredibleDotProductTaskManagerCaller // Generic read-only contract binding to access the raw methods on
}

// ContractIncredibleDotProductTaskManagerTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type ContractIncredibleDotProductTaskManagerTransactorRaw struct {
	Contract *ContractIncredibleDotProductTaskManagerTransactor // Generic write-only contract binding to access the raw methods on
}

// NewContractIncredibleDotProductTaskManager creates a new instance of ContractIncredibleDotProductTaskManager, bound to a specific deployed contract.
func NewContractIncredibleDotProductTaskManager(address common.Address, backend bind.ContractBackend) (*ContractIncredibleDotProductTaskManager, error) {
	contract, err := bindContractIncredibleDotProductTaskManager(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManager{ContractIncredibleDotProductTaskManagerCaller: ContractIncredibleDotProductTaskManagerCaller{contract: contract}, ContractIncredibleDotProductTaskManagerTransactor: ContractIncredibleDotProductTaskManagerTransactor{contract: contract}, ContractIncredibleDotProductTaskManagerFilterer: ContractIncredibleDotProductTaskManagerFilterer{contract: contract}}, nil
}

// NewContractIncredibleDotProductTaskManagerCaller creates a new read-only instance of ContractIncredibleDotProductTaskManager, bound to a specific deployed contract.
func NewContractIncredibleDotProductTaskManagerCaller(address common.Address, caller bind.ContractCaller) (*ContractIncredibleDotProductTaskManagerCaller, error) {
	contract, err := bindContractIncredibleDotProductTaskManager(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerCaller{contract: contract}, nil
}

// NewContractIncredibleDotProductTaskManagerTransactor creates a new write-only instance of ContractIncredibleDotProductTaskManager, bound to a specific deployed contract.
func NewContractIncredibleDotProductTaskManagerTransactor(address common.Address, transactor bind.ContractTransactor) (*ContractIncredibleDotProductTaskManagerTransactor, error) {
	contract, err := bindContractIncredibleDotProductTaskManager(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerTransactor{contract: contract}, nil
}

// NewContractIncredibleDotProductTaskManagerFilterer creates a new log filterer instance of ContractIncredibleDotProductTaskManager, bound to a specific deployed contract.
func NewContractIncredibleDotProductTaskManagerFilterer(address common.Address, filterer bind.ContractFilterer) (*ContractIncredibleDotProductTaskManagerFilterer, error) {
	contract, err := bindContractIncredibleDotProductTaskManager(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerFilterer{contract: contract}, nil
}

// bindContractIncredibleDotProductTaskManager binds a generic wrapper to an already deployed contract.
func bindContractIncredibleDotProductTaskManager(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := ContractIncredibleDotProductTaskManagerMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractIncredibleDotProductTaskManager.Contract.ContractIncredibleDotProductTaskManagerCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.ContractIncredibleDotProductTaskManagerTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.ContractIncredibleDotProductTaskManagerTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractIncredibleDotProductTaskManager.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.contract.Transact(opts, method, params...)
}

// TASKCHALLENGEWINDOWBLOCK is a free data retrieval call binding the contract method 0xf63c5bab.
//
// Solidity: function TASK_CHALLENGE_WINDOW_BLOCK() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) TASKCHALLENGEWINDOWBLOCK(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "TASK_CHALLENGE_WINDOW_BLOCK")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// TASKCHALLENGEWINDOWBLOCK is a free data retrieval call binding the contract method 0xf63c5bab.
//
// Solidity: function TASK_CHALLENGE_WINDOW_BLOCK() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) TASKCHALLENGEWINDOWBLOCK() (uint32, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TASKCHALLENGEWINDOWBLOCK(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// TASKCHALLENGEWINDOWBLOCK is a free data retrieval call binding the contract method 0xf63c5bab.
//
// Solidity: function TASK_CHALLENGE_WINDOW_BLOCK() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) TASKCHALLENGEWINDOWBLOCK() (uint32, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TASKCHALLENGEWINDOWBLOCK(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// TASKRESPONSEWINDOWBLOCK is a free data retrieval call binding the contract method 0x1ad43189.
//
// Solidity: function TASK_RESPONSE_WINDOW_BLOCK() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) TASKRESPONSEWINDOWBLOCK(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "TASK_RESPONSE_WINDOW_BLOCK")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// TASKRESPONSEWINDOWBLOCK is a free data retrieval call binding the contract method 0x1ad43189.
//
// Solidity: function TASK_RESPONSE_WINDOW_BLOCK() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) TASKRESPONSEWINDOWBLOCK() (uint32, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TASKRESPONSEWINDOWBLOCK(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// TASKRESPONSEWINDOWBLOCK is a free data retrieval call binding the contract method 0x1ad43189.
//
// Solidity: function TASK_RESPONSE_WINDOW_BLOCK() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) TASKRESPONSEWINDOWBLOCK() (uint32, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TASKRESPONSEWINDOWBLOCK(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// WADSTOSLASH is a free data retrieval call binding the contract method 0x5a2d7f02.
//
// Solidity: function WADS_TO_SLASH() view returns(uint256)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) WADSTOSLASH(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "WADS_TO_SLASH")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// WADSTOSLASH is a free data retrieval call binding the contract method 0x5a2d7f02.
//
// Solidity: function WADS_TO_SLASH() view returns(uint256)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) WADSTOSLASH() (*big.Int, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.WADSTOSLASH(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// WADSTOSLASH is a free data retrieval call binding the contract method 0x5a2d7f02.
//
// Solidity: function WADS_TO_SLASH() view returns(uint256)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) WADSTOSLASH() (*big.Int, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.WADSTOSLASH(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// Aggregator is a free data retrieval call binding the contract method 0x245a7bfc.
//
// Solidity: function aggregator() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) Aggregator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "aggregator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Aggregator is a free data retrieval call binding the contract method 0x245a7bfc.
//
// Solidity: function aggregator() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) Aggregator() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Aggregator(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// Aggregator is a free data retrieval call binding the contract method 0x245a7bfc.
//
// Solidity: function aggregator() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) Aggregator() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Aggregator(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// AllTaskHashes is a free data retrieval call binding the contract method 0x2d89f6fc.
//
// Solidity: function allTaskHashes(uint32 ) view returns(bytes32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) AllTaskHashes(opts *bind.CallOpts, arg0 uint32) ([32]byte, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "allTaskHashes", arg0)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// AllTaskHashes is a free data retrieval call binding the contract method 0x2d89f6fc.
//
// Solidity: function allTaskHashes(uint32 ) view returns(bytes32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) AllTaskHashes(arg0 uint32) ([32]byte, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.AllTaskHashes(&_ContractIncredibleDotProductTaskManager.CallOpts, arg0)
}

// AllTaskHashes is a free data retrieval call binding the contract method 0x2d89f6fc.
//
// Solidity: function allTaskHashes(uint32 ) view returns(bytes32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) AllTaskHashes(arg0 uint32) ([32]byte, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.AllTaskHashes(&_ContractIncredibleDotProductTaskManager.CallOpts, arg0)
}

// AllTaskResponses is a free data retrieval call binding the contract method 0x2cb223d5.
//
// Solidity: function allTaskResponses(uint32 ) view returns(bytes32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) AllTaskResponses(opts *bind.CallOpts, arg0 uint32) ([32]byte, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "allTaskResponses", arg0)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// AllTaskResponses is a free data retrieval call binding the contract method 0x2cb223d5.
//
// Solidity: function allTaskResponses(uint32 ) view returns(bytes32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) AllTaskResponses(arg0 uint32) ([32]byte, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.AllTaskResponses(&_ContractIncredibleDotProductTaskManager.CallOpts, arg0)
}

// AllTaskResponses is a free data retrieval call binding the contract method 0x2cb223d5.
//
// Solidity: function allTaskResponses(uint32 ) view returns(bytes32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) AllTaskResponses(arg0 uint32) ([32]byte, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.AllTaskResponses(&_ContractIncredibleDotProductTaskManager.CallOpts, arg0)
}

// AllocationManager is a free data retrieval call binding the contract method 0xca8aa7c7.
//
// Solidity: function allocationManager() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) AllocationManager(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "allocationManager")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// AllocationManager is a free data retrieval call binding the contract method 0xca8aa7c7.
//
// Solidity: function allocationManager() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) AllocationManager() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.AllocationManager(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// AllocationManager is a free data retrieval call binding the contract method 0xca8aa7c7.
//
// Solidity: function allocationManager() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) AllocationManager() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.AllocationManager(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// BlsApkRegistry is a free data retrieval call binding the contract method 0x5df45946.
//
// Solidity: function blsApkRegistry() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) BlsApkRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "blsApkRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// BlsApkRegistry is a free data retrieval call binding the contract method 0x5df45946.
//
// Solidity: function blsApkRegistry() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) BlsApkRegistry() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.BlsApkRegistry(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// BlsApkRegistry is a free data retrieval call binding the contract method 0x5df45946.
//
// Solidity: function blsApkRegistry() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) BlsApkRegistry() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.BlsApkRegistry(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]), bytes32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) CheckSignatures(opts *bind.CallOpts, msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, params IBLSSignatureCheckerTypesNonSignerStakesAndSignature) (IBLSSignatureCheckerTypesQuorumStakeTotals, [32]byte, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "checkSignatures", msgHash, quorumNumbers, referenceBlockNumber, params)

	if err != nil {
		return *new(IBLSSignatureCheckerTypesQuorumStakeTotals), *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new(IBLSSignatureCheckerTypesQuorumStakeTotals)).(*IBLSSignatureCheckerTypesQuorumStakeTotals)
	out1 := *abi.ConvertType(out[1], new([32]byte)).(*[32]byte)

	return out0, out1, err

}

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]), bytes32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) CheckSignatures(msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, params IBLSSignatureCheckerTypesNonSignerStakesAndSignature) (IBLSSignatureCheckerTypesQuorumStakeTotals, [32]byte, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.CheckSignatures(&_ContractIncredibleDotProductTaskManager.CallOpts, msgHash, quorumNumbers, referenceBlockNumber, params)
}

// CheckSignatures is a free data retrieval call binding the contract method 0x6efb4636.
//
// Solidity: function checkSignatures(bytes32 msgHash, bytes quorumNumbers, uint32 referenceBlockNumber, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) params) view returns((uint96[],uint96[]), bytes32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) CheckSignatures(msgHash [32]byte, quorumNumbers []byte, referenceBlockNumber uint32, params IBLSSignatureCheckerTypesNonSignerStakesAndSignature) (IBLSSignatureCheckerTypesQuorumStakeTotals, [32]byte, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.CheckSignatures(&_ContractIncredibleDotProductTaskManager.CallOpts, msgHash, quorumNumbers, referenceBlockNumber, params)
}

// Delegation is a free data retrieval call binding the contract method 0xdf5cf723.
//
// Solidity: function delegation() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) Delegation(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "delegation")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Delegation is a free data retrieval call binding the contract method 0xdf5cf723.
//
// Solidity: function delegation() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) Delegation() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Delegation(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// Delegation is a free data retrieval call binding the contract method 0xdf5cf723.
//
// Solidity: function delegation() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) Delegation() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Delegation(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// Generator is a free data retrieval call binding the contract method 0x7afa1eed.
//
// Solidity: function generator() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) Generator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "generator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Generator is a free data retrieval call binding the contract method 0x7afa1eed.
//
// Solidity: function generator() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) Generator() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Generator(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// Generator is a free data retrieval call binding the contract method 0x7afa1eed.
//
// Solidity: function generator() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) Generator() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Generator(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// GetBatchOperatorFromId is a free data retrieval call binding the contract method 0x4d2b57fe.
//
// Solidity: function getBatchOperatorFromId(address registryCoordinator, bytes32[] operatorIds) view returns(address[] operators)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) GetBatchOperatorFromId(opts *bind.CallOpts, registryCoordinator common.Address, operatorIds [][32]byte) ([]common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "getBatchOperatorFromId", registryCoordinator, operatorIds)

	if err != nil {
		return *new([]common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new([]common.Address)).(*[]common.Address)

	return out0, err

}

// GetBatchOperatorFromId is a free data retrieval call binding the contract method 0x4d2b57fe.
//
// Solidity: function getBatchOperatorFromId(address registryCoordinator, bytes32[] operatorIds) view returns(address[] operators)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) GetBatchOperatorFromId(registryCoordinator common.Address, operatorIds [][32]byte) ([]common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetBatchOperatorFromId(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, operatorIds)
}

// GetBatchOperatorFromId is a free data retrieval call binding the contract method 0x4d2b57fe.
//
// Solidity: function getBatchOperatorFromId(address registryCoordinator, bytes32[] operatorIds) view returns(address[] operators)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) GetBatchOperatorFromId(registryCoordinator common.Address, operatorIds [][32]byte) ([]common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetBatchOperatorFromId(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, operatorIds)
}

// GetBatchOperatorId is a free data retrieval call binding the contract method 0x31b36bd9.
//
// Solidity: function getBatchOperatorId(address registryCoordinator, address[] operators) view returns(bytes32[] operatorIds)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) GetBatchOperatorId(opts *bind.CallOpts, registryCoordinator common.Address, operators []common.Address) ([][32]byte, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "getBatchOperatorId", registryCoordinator, operators)

	if err != nil {
		return *new([][32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([][32]byte)).(*[][32]byte)

	return out0, err

}

// GetBatchOperatorId is a free data retrieval call binding the contract method 0x31b36bd9.
//
// Solidity: function getBatchOperatorId(address registryCoordinator, address[] operators) view returns(bytes32[] operatorIds)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) GetBatchOperatorId(registryCoordinator common.Address, operators []common.Address) ([][32]byte, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetBatchOperatorId(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, operators)
}

// GetBatchOperatorId is a free data retrieval call binding the contract method 0x31b36bd9.
//
// Solidity: function getBatchOperatorId(address registryCoordinator, address[] operators) view returns(bytes32[] operatorIds)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) GetBatchOperatorId(registryCoordinator common.Address, operators []common.Address) ([][32]byte, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetBatchOperatorId(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, operators)
}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) GetCheckSignaturesIndices(opts *bind.CallOpts, registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (OperatorStateRetrieverCheckSignaturesIndices, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "getCheckSignaturesIndices", registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)

	if err != nil {
		return *new(OperatorStateRetrieverCheckSignaturesIndices), err
	}

	out0 := *abi.ConvertType(out[0], new(OperatorStateRetrieverCheckSignaturesIndices)).(*OperatorStateRetrieverCheckSignaturesIndices)

	return out0, err

}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) GetCheckSignaturesIndices(registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (OperatorStateRetrieverCheckSignaturesIndices, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetCheckSignaturesIndices(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)
}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) GetCheckSignaturesIndices(registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (OperatorStateRetrieverCheckSignaturesIndices, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetCheckSignaturesIndices(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)
}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((address,bytes32,uint96)[][])
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) GetOperatorState(opts *bind.CallOpts, registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]OperatorStateRetrieverOperator, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "getOperatorState", registryCoordinator, quorumNumbers, blockNumber)

	if err != nil {
		return *new([][]OperatorStateRetrieverOperator), err
	}

	out0 := *abi.ConvertType(out[0], new([][]OperatorStateRetrieverOperator)).(*[][]OperatorStateRetrieverOperator)

	return out0, err

}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((address,bytes32,uint96)[][])
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) GetOperatorState(registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]OperatorStateRetrieverOperator, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetOperatorState(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, quorumNumbers, blockNumber)
}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((address,bytes32,uint96)[][])
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) GetOperatorState(registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]OperatorStateRetrieverOperator, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetOperatorState(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, quorumNumbers, blockNumber)
}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (address,bytes32,uint96)[][])
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) GetOperatorState0(opts *bind.CallOpts, registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]OperatorStateRetrieverOperator, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "getOperatorState0", registryCoordinator, operatorId, blockNumber)

	if err != nil {
		return *new(*big.Int), *new([][]OperatorStateRetrieverOperator), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)
	out1 := *abi.ConvertType(out[1], new([][]OperatorStateRetrieverOperator)).(*[][]OperatorStateRetrieverOperator)

	return out0, out1, err

}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (address,bytes32,uint96)[][])
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) GetOperatorState0(registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]OperatorStateRetrieverOperator, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetOperatorState0(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, operatorId, blockNumber)
}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (address,bytes32,uint96)[][])
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) GetOperatorState0(registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]OperatorStateRetrieverOperator, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetOperatorState0(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, operatorId, blockNumber)
}

// GetQuorumBitmapsAtBlockNumber is a free data retrieval call binding the contract method 0x5c155662.
//
// Solidity: function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] operatorIds, uint32 blockNumber) view returns(uint256[])
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) GetQuorumBitmapsAtBlockNumber(opts *bind.CallOpts, registryCoordinator common.Address, operatorIds [][32]byte, blockNumber uint32) ([]*big.Int, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "getQuorumBitmapsAtBlockNumber", registryCoordinator, operatorIds, blockNumber)

	if err != nil {
		return *new([]*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new([]*big.Int)).(*[]*big.Int)

	return out0, err

}

// GetQuorumBitmapsAtBlockNumber is a free data retrieval call binding the contract method 0x5c155662.
//
// Solidity: function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] operatorIds, uint32 blockNumber) view returns(uint256[])
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) GetQuorumBitmapsAtBlockNumber(registryCoordinator common.Address, operatorIds [][32]byte, blockNumber uint32) ([]*big.Int, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetQuorumBitmapsAtBlockNumber(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, operatorIds, blockNumber)
}

// GetQuorumBitmapsAtBlockNumber is a free data retrieval call binding the contract method 0x5c155662.
//
// Solidity: function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] operatorIds, uint32 blockNumber) view returns(uint256[])
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) GetQuorumBitmapsAtBlockNumber(registryCoordinator common.Address, operatorIds [][32]byte, blockNumber uint32) ([]*big.Int, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetQuorumBitmapsAtBlockNumber(&_ContractIncredibleDotProductTaskManager.CallOpts, registryCoordinator, operatorIds, blockNumber)
}

// GetTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xf5c9899d.
//
// Solidity: function getTaskResponseWindowBlock() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) GetTaskResponseWindowBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "getTaskResponseWindowBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// GetTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xf5c9899d.
//
// Solidity: function getTaskResponseWindowBlock() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) GetTaskResponseWindowBlock() (uint32, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetTaskResponseWindowBlock(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// GetTaskResponseWindowBlock is a free data retrieval call binding the contract method 0xf5c9899d.
//
// Solidity: function getTaskResponseWindowBlock() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) GetTaskResponseWindowBlock() (uint32, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.GetTaskResponseWindowBlock(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// InstantSlasher is a free data retrieval call binding the contract method 0x9b290e98.
//
// Solidity: function instantSlasher() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) InstantSlasher(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "instantSlasher")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// InstantSlasher is a free data retrieval call binding the contract method 0x9b290e98.
//
// Solidity: function instantSlasher() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) InstantSlasher() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.InstantSlasher(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// InstantSlasher is a free data retrieval call binding the contract method 0x9b290e98.
//
// Solidity: function instantSlasher() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) InstantSlasher() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.InstantSlasher(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// LatestTaskNum is a free data retrieval call binding the contract method 0x8b00ce7c.
//
// Solidity: function latestTaskNum() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) LatestTaskNum(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "latestTaskNum")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestTaskNum is a free data retrieval call binding the contract method 0x8b00ce7c.
//
// Solidity: function latestTaskNum() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) LatestTaskNum() (uint32, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.LatestTaskNum(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// LatestTaskNum is a free data retrieval call binding the contract method 0x8b00ce7c.
//
// Solidity: function latestTaskNum() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) LatestTaskNum() (uint32, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.LatestTaskNum(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) Owner() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Owner(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) Owner() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Owner(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) Paused(opts *bind.CallOpts, index uint8) (bool, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "paused", index)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) Paused(index uint8) (bool, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Paused(&_ContractIncredibleDotProductTaskManager.CallOpts, index)
}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) Paused(index uint8) (bool, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Paused(&_ContractIncredibleDotProductTaskManager.CallOpts, index)
}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) Paused0(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "paused0")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) Paused0() (*big.Int, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Paused0(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) Paused0() (*big.Int, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Paused0(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) PauserRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "pauserRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) PauserRegistry() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.PauserRegistry(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) PauserRegistry() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.PauserRegistry(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) RegistryCoordinator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "registryCoordinator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) RegistryCoordinator() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.RegistryCoordinator(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// RegistryCoordinator is a free data retrieval call binding the contract method 0x6d14a987.
//
// Solidity: function registryCoordinator() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) RegistryCoordinator() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.RegistryCoordinator(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// ServiceManager is a free data retrieval call binding the contract method 0x3998fdd3.
//
// Solidity: function serviceManager() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) ServiceManager(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "serviceManager")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// ServiceManager is a free data retrieval call binding the contract method 0x3998fdd3.
//
// Solidity: function serviceManager() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) ServiceManager() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.ServiceManager(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// ServiceManager is a free data retrieval call binding the contract method 0x3998fdd3.
//
// Solidity: function serviceManager() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) ServiceManager() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.ServiceManager(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) StakeRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "stakeRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) StakeRegistry() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.StakeRegistry(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// StakeRegistry is a free data retrieval call binding the contract method 0x68304835.
//
// Solidity: function stakeRegistry() view returns(address)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) StakeRegistry() (common.Address, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.StakeRegistry(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// StaleStakesForbidden is a free data retrieval call binding the contract method 0xb98d0908.
//
// Solidity: function staleStakesForbidden() view returns(bool)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) StaleStakesForbidden(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "staleStakesForbidden")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// StaleStakesForbidden is a free data retrieval call binding the contract method 0xb98d0908.
//
// Solidity: function staleStakesForbidden() view returns(bool)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) StaleStakesForbidden() (bool, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.StaleStakesForbidden(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// StaleStakesForbidden is a free data retrieval call binding the contract method 0xb98d0908.
//
// Solidity: function staleStakesForbidden() view returns(bool)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) StaleStakesForbidden() (bool, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.StaleStakesForbidden(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// TaskNumber is a free data retrieval call binding the contract method 0x72d18e8d.
//
// Solidity: function taskNumber() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) TaskNumber(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "taskNumber")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// TaskNumber is a free data retrieval call binding the contract method 0x72d18e8d.
//
// Solidity: function taskNumber() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) TaskNumber() (uint32, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TaskNumber(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// TaskNumber is a free data retrieval call binding the contract method 0x72d18e8d.
//
// Solidity: function taskNumber() view returns(uint32)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) TaskNumber() (uint32, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TaskNumber(&_ContractIncredibleDotProductTaskManager.CallOpts)
}

// TaskSuccesfullyChallenged is a free data retrieval call binding the contract method 0x5decc3f5.
//
// Solidity: function taskSuccesfullyChallenged(uint32 ) view returns(bool)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) TaskSuccesfullyChallenged(opts *bind.CallOpts, arg0 uint32) (bool, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "taskSuccesfullyChallenged", arg0)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// TaskSuccesfullyChallenged is a free data retrieval call binding the contract method 0x5decc3f5.
//
// Solidity: function taskSuccesfullyChallenged(uint32 ) view returns(bool)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) TaskSuccesfullyChallenged(arg0 uint32) (bool, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TaskSuccesfullyChallenged(&_ContractIncredibleDotProductTaskManager.CallOpts, arg0)
}

// TaskSuccesfullyChallenged is a free data retrieval call binding the contract method 0x5decc3f5.
//
// Solidity: function taskSuccesfullyChallenged(uint32 ) view returns(bool)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) TaskSuccesfullyChallenged(arg0 uint32) (bool, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TaskSuccesfullyChallenged(&_ContractIncredibleDotProductTaskManager.CallOpts, arg0)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCaller) TrySignatureAndApkVerification(opts *bind.CallOpts, msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductTaskManager.contract.Call(opts, &out, "trySignatureAndApkVerification", msgHash, apk, apkG2, sigma)

	outstruct := new(struct {
		PairingSuccessful bool
		SiganatureIsValid bool
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.PairingSuccessful = *abi.ConvertType(out[0], new(bool)).(*bool)
	outstruct.SiganatureIsValid = *abi.ConvertType(out[1], new(bool)).(*bool)

	return *outstruct, err

}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TrySignatureAndApkVerification(&_ContractIncredibleDotProductTaskManager.CallOpts, msgHash, apk, apkG2, sigma)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerCallerSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TrySignatureAndApkVerification(&_ContractIncredibleDotProductTaskManager.CallOpts, msgHash, apk, apkG2, sigma)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0x01863067.
//
// Solidity: function createNewTask((uint256[],uint256[]) points, uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactor) CreateNewTask(opts *bind.TransactOpts, points IIncredibleDotProductTaskManagerDotProductInput, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.contract.Transact(opts, "createNewTask", points, quorumThresholdPercentage, quorumNumbers)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0x01863067.
//
// Solidity: function createNewTask((uint256[],uint256[]) points, uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) CreateNewTask(points IIncredibleDotProductTaskManagerDotProductInput, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.CreateNewTask(&_ContractIncredibleDotProductTaskManager.TransactOpts, points, quorumThresholdPercentage, quorumNumbers)
}

// CreateNewTask is a paid mutator transaction binding the contract method 0x01863067.
//
// Solidity: function createNewTask((uint256[],uint256[]) points, uint32 quorumThresholdPercentage, bytes quorumNumbers) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorSession) CreateNewTask(points IIncredibleDotProductTaskManagerDotProductInput, quorumThresholdPercentage uint32, quorumNumbers []byte) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.CreateNewTask(&_ContractIncredibleDotProductTaskManager.TransactOpts, points, quorumThresholdPercentage, quorumNumbers)
}

// Initialize is a paid mutator transaction binding the contract method 0xcc2a9a5b.
//
// Solidity: function initialize(address initialOwner, address _aggregator, address _generator, address _allocationManager, address _slasher, address _serviceManager) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactor) Initialize(opts *bind.TransactOpts, initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allocationManager common.Address, _slasher common.Address, _serviceManager common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.contract.Transact(opts, "initialize", initialOwner, _aggregator, _generator, _allocationManager, _slasher, _serviceManager)
}

// Initialize is a paid mutator transaction binding the contract method 0xcc2a9a5b.
//
// Solidity: function initialize(address initialOwner, address _aggregator, address _generator, address _allocationManager, address _slasher, address _serviceManager) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) Initialize(initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allocationManager common.Address, _slasher common.Address, _serviceManager common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Initialize(&_ContractIncredibleDotProductTaskManager.TransactOpts, initialOwner, _aggregator, _generator, _allocationManager, _slasher, _serviceManager)
}

// Initialize is a paid mutator transaction binding the contract method 0xcc2a9a5b.
//
// Solidity: function initialize(address initialOwner, address _aggregator, address _generator, address _allocationManager, address _slasher, address _serviceManager) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorSession) Initialize(initialOwner common.Address, _aggregator common.Address, _generator common.Address, _allocationManager common.Address, _slasher common.Address, _serviceManager common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Initialize(&_ContractIncredibleDotProductTaskManager.TransactOpts, initialOwner, _aggregator, _generator, _allocationManager, _slasher, _serviceManager)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactor) Pause(opts *bind.TransactOpts, newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.contract.Transact(opts, "pause", newPausedStatus)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) Pause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Pause(&_ContractIncredibleDotProductTaskManager.TransactOpts, newPausedStatus)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorSession) Pause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Pause(&_ContractIncredibleDotProductTaskManager.TransactOpts, newPausedStatus)
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactor) PauseAll(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.contract.Transact(opts, "pauseAll")
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) PauseAll() (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.PauseAll(&_ContractIncredibleDotProductTaskManager.TransactOpts)
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorSession) PauseAll() (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.PauseAll(&_ContractIncredibleDotProductTaskManager.TransactOpts)
}

// RaiseAndResolveChallenge is a paid mutator transaction binding the contract method 0xb490bb41.
//
// Solidity: function raiseAndResolveChallenge(((uint256[],uint256[]),uint32,bytes,uint32) task, (uint32,uint256) taskResponse, (uint32,bytes32) taskResponseMetadata, (uint256,uint256)[] pubkeysOfNonSigningOperators) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactor) RaiseAndResolveChallenge(opts *bind.TransactOpts, task IIncredibleDotProductTaskManagerTask, taskResponse IIncredibleDotProductTaskManagerTaskResponse, taskResponseMetadata IIncredibleDotProductTaskManagerTaskResponseMetadata, pubkeysOfNonSigningOperators []BN254G1Point) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.contract.Transact(opts, "raiseAndResolveChallenge", task, taskResponse, taskResponseMetadata, pubkeysOfNonSigningOperators)
}

// RaiseAndResolveChallenge is a paid mutator transaction binding the contract method 0xb490bb41.
//
// Solidity: function raiseAndResolveChallenge(((uint256[],uint256[]),uint32,bytes,uint32) task, (uint32,uint256) taskResponse, (uint32,bytes32) taskResponseMetadata, (uint256,uint256)[] pubkeysOfNonSigningOperators) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) RaiseAndResolveChallenge(task IIncredibleDotProductTaskManagerTask, taskResponse IIncredibleDotProductTaskManagerTaskResponse, taskResponseMetadata IIncredibleDotProductTaskManagerTaskResponseMetadata, pubkeysOfNonSigningOperators []BN254G1Point) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.RaiseAndResolveChallenge(&_ContractIncredibleDotProductTaskManager.TransactOpts, task, taskResponse, taskResponseMetadata, pubkeysOfNonSigningOperators)
}

// RaiseAndResolveChallenge is a paid mutator transaction binding the contract method 0xb490bb41.
//
// Solidity: function raiseAndResolveChallenge(((uint256[],uint256[]),uint32,bytes,uint32) task, (uint32,uint256) taskResponse, (uint32,bytes32) taskResponseMetadata, (uint256,uint256)[] pubkeysOfNonSigningOperators) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorSession) RaiseAndResolveChallenge(task IIncredibleDotProductTaskManagerTask, taskResponse IIncredibleDotProductTaskManagerTaskResponse, taskResponseMetadata IIncredibleDotProductTaskManagerTaskResponseMetadata, pubkeysOfNonSigningOperators []BN254G1Point) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.RaiseAndResolveChallenge(&_ContractIncredibleDotProductTaskManager.TransactOpts, task, taskResponse, taskResponseMetadata, pubkeysOfNonSigningOperators)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.RenounceOwnership(&_ContractIncredibleDotProductTaskManager.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.RenounceOwnership(&_ContractIncredibleDotProductTaskManager.TransactOpts)
}

// RespondToTask is a paid mutator transaction binding the contract method 0xc9b14797.
//
// Solidity: function respondToTask(((uint256[],uint256[]),uint32,bytes,uint32) task, (uint32,uint256) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactor) RespondToTask(opts *bind.TransactOpts, task IIncredibleDotProductTaskManagerTask, taskResponse IIncredibleDotProductTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerTypesNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.contract.Transact(opts, "respondToTask", task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToTask is a paid mutator transaction binding the contract method 0xc9b14797.
//
// Solidity: function respondToTask(((uint256[],uint256[]),uint32,bytes,uint32) task, (uint32,uint256) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) RespondToTask(task IIncredibleDotProductTaskManagerTask, taskResponse IIncredibleDotProductTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerTypesNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.RespondToTask(&_ContractIncredibleDotProductTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// RespondToTask is a paid mutator transaction binding the contract method 0xc9b14797.
//
// Solidity: function respondToTask(((uint256[],uint256[]),uint32,bytes,uint32) task, (uint32,uint256) taskResponse, (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) nonSignerStakesAndSignature) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorSession) RespondToTask(task IIncredibleDotProductTaskManagerTask, taskResponse IIncredibleDotProductTaskManagerTaskResponse, nonSignerStakesAndSignature IBLSSignatureCheckerTypesNonSignerStakesAndSignature) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.RespondToTask(&_ContractIncredibleDotProductTaskManager.TransactOpts, task, taskResponse, nonSignerStakesAndSignature)
}

// SetStaleStakesForbidden is a paid mutator transaction binding the contract method 0x416c7e5e.
//
// Solidity: function setStaleStakesForbidden(bool value) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactor) SetStaleStakesForbidden(opts *bind.TransactOpts, value bool) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.contract.Transact(opts, "setStaleStakesForbidden", value)
}

// SetStaleStakesForbidden is a paid mutator transaction binding the contract method 0x416c7e5e.
//
// Solidity: function setStaleStakesForbidden(bool value) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) SetStaleStakesForbidden(value bool) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.SetStaleStakesForbidden(&_ContractIncredibleDotProductTaskManager.TransactOpts, value)
}

// SetStaleStakesForbidden is a paid mutator transaction binding the contract method 0x416c7e5e.
//
// Solidity: function setStaleStakesForbidden(bool value) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorSession) SetStaleStakesForbidden(value bool) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.SetStaleStakesForbidden(&_ContractIncredibleDotProductTaskManager.TransactOpts, value)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TransferOwnership(&_ContractIncredibleDotProductTaskManager.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.TransferOwnership(&_ContractIncredibleDotProductTaskManager.TransactOpts, newOwner)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactor) Unpause(opts *bind.TransactOpts, newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.contract.Transact(opts, "unpause", newPausedStatus)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerSession) Unpause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Unpause(&_ContractIncredibleDotProductTaskManager.TransactOpts, newPausedStatus)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerTransactorSession) Unpause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractIncredibleDotProductTaskManager.Contract.Unpause(&_ContractIncredibleDotProductTaskManager.TransactOpts, newPausedStatus)
}

// ContractIncredibleDotProductTaskManagerInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerInitializedIterator struct {
	Event *ContractIncredibleDotProductTaskManagerInitialized // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractIncredibleDotProductTaskManagerInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductTaskManagerInitialized)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractIncredibleDotProductTaskManagerInitialized)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractIncredibleDotProductTaskManagerInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductTaskManagerInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductTaskManagerInitialized represents a Initialized event raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) FilterInitialized(opts *bind.FilterOpts) (*ContractIncredibleDotProductTaskManagerInitializedIterator, error) {

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerInitializedIterator{contract: _ContractIncredibleDotProductTaskManager.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductTaskManagerInitialized) (event.Subscription, error) {

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductTaskManagerInitialized)
				if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "Initialized", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseInitialized is a log parse operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) ParseInitialized(log types.Log) (*ContractIncredibleDotProductTaskManagerInitialized, error) {
	event := new(ContractIncredibleDotProductTaskManagerInitialized)
	if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductTaskManagerNewTaskCreatedIterator is returned from FilterNewTaskCreated and is used to iterate over the raw logs and unpacked data for NewTaskCreated events raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerNewTaskCreatedIterator struct {
	Event *ContractIncredibleDotProductTaskManagerNewTaskCreated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractIncredibleDotProductTaskManagerNewTaskCreatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductTaskManagerNewTaskCreated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractIncredibleDotProductTaskManagerNewTaskCreated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractIncredibleDotProductTaskManagerNewTaskCreatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductTaskManagerNewTaskCreatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductTaskManagerNewTaskCreated represents a NewTaskCreated event raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerNewTaskCreated struct {
	TaskIndex uint32
	Task      IIncredibleDotProductTaskManagerTask
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterNewTaskCreated is a free log retrieval operation binding the contract event 0xf3d0298607cbbde38baba3c3915d277f1ffd80a384095a02a4a4d7c082ae6cd9.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, ((uint256[],uint256[]),uint32,bytes,uint32) task)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) FilterNewTaskCreated(opts *bind.FilterOpts, taskIndex []uint32) (*ContractIncredibleDotProductTaskManagerNewTaskCreatedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.FilterLogs(opts, "NewTaskCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerNewTaskCreatedIterator{contract: _ContractIncredibleDotProductTaskManager.contract, event: "NewTaskCreated", logs: logs, sub: sub}, nil
}

// WatchNewTaskCreated is a free log subscription operation binding the contract event 0xf3d0298607cbbde38baba3c3915d277f1ffd80a384095a02a4a4d7c082ae6cd9.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, ((uint256[],uint256[]),uint32,bytes,uint32) task)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) WatchNewTaskCreated(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductTaskManagerNewTaskCreated, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.WatchLogs(opts, "NewTaskCreated", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductTaskManagerNewTaskCreated)
				if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "NewTaskCreated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseNewTaskCreated is a log parse operation binding the contract event 0xf3d0298607cbbde38baba3c3915d277f1ffd80a384095a02a4a4d7c082ae6cd9.
//
// Solidity: event NewTaskCreated(uint32 indexed taskIndex, ((uint256[],uint256[]),uint32,bytes,uint32) task)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) ParseNewTaskCreated(log types.Log) (*ContractIncredibleDotProductTaskManagerNewTaskCreated, error) {
	event := new(ContractIncredibleDotProductTaskManagerNewTaskCreated)
	if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "NewTaskCreated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductTaskManagerOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerOwnershipTransferredIterator struct {
	Event *ContractIncredibleDotProductTaskManagerOwnershipTransferred // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractIncredibleDotProductTaskManagerOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductTaskManagerOwnershipTransferred)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractIncredibleDotProductTaskManagerOwnershipTransferred)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractIncredibleDotProductTaskManagerOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductTaskManagerOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductTaskManagerOwnershipTransferred represents a OwnershipTransferred event raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*ContractIncredibleDotProductTaskManagerOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerOwnershipTransferredIterator{contract: _ContractIncredibleDotProductTaskManager.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductTaskManagerOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductTaskManagerOwnershipTransferred)
				if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnershipTransferred is a log parse operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) ParseOwnershipTransferred(log types.Log) (*ContractIncredibleDotProductTaskManagerOwnershipTransferred, error) {
	event := new(ContractIncredibleDotProductTaskManagerOwnershipTransferred)
	if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductTaskManagerPausedIterator is returned from FilterPaused and is used to iterate over the raw logs and unpacked data for Paused events raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerPausedIterator struct {
	Event *ContractIncredibleDotProductTaskManagerPaused // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractIncredibleDotProductTaskManagerPausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductTaskManagerPaused)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractIncredibleDotProductTaskManagerPaused)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractIncredibleDotProductTaskManagerPausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductTaskManagerPausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductTaskManagerPaused represents a Paused event raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerPaused struct {
	Account         common.Address
	NewPausedStatus *big.Int
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterPaused is a free log retrieval operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) FilterPaused(opts *bind.FilterOpts, account []common.Address) (*ContractIncredibleDotProductTaskManagerPausedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.FilterLogs(opts, "Paused", accountRule)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerPausedIterator{contract: _ContractIncredibleDotProductTaskManager.contract, event: "Paused", logs: logs, sub: sub}, nil
}

// WatchPaused is a free log subscription operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) WatchPaused(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductTaskManagerPaused, account []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.WatchLogs(opts, "Paused", accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductTaskManagerPaused)
				if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "Paused", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParsePaused is a log parse operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) ParsePaused(log types.Log) (*ContractIncredibleDotProductTaskManagerPaused, error) {
	event := new(ContractIncredibleDotProductTaskManagerPaused)
	if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "Paused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdateIterator is returned from FilterStaleStakesForbiddenUpdate and is used to iterate over the raw logs and unpacked data for StaleStakesForbiddenUpdate events raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdateIterator struct {
	Event *ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdate // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdateIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdate)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdate)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdateIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdateIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdate represents a StaleStakesForbiddenUpdate event raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdate struct {
	Value bool
	Raw   types.Log // Blockchain specific contextual infos
}

// FilterStaleStakesForbiddenUpdate is a free log retrieval operation binding the contract event 0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc.
//
// Solidity: event StaleStakesForbiddenUpdate(bool value)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) FilterStaleStakesForbiddenUpdate(opts *bind.FilterOpts) (*ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdateIterator, error) {

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.FilterLogs(opts, "StaleStakesForbiddenUpdate")
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdateIterator{contract: _ContractIncredibleDotProductTaskManager.contract, event: "StaleStakesForbiddenUpdate", logs: logs, sub: sub}, nil
}

// WatchStaleStakesForbiddenUpdate is a free log subscription operation binding the contract event 0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc.
//
// Solidity: event StaleStakesForbiddenUpdate(bool value)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) WatchStaleStakesForbiddenUpdate(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdate) (event.Subscription, error) {

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.WatchLogs(opts, "StaleStakesForbiddenUpdate")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdate)
				if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "StaleStakesForbiddenUpdate", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseStaleStakesForbiddenUpdate is a log parse operation binding the contract event 0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc.
//
// Solidity: event StaleStakesForbiddenUpdate(bool value)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) ParseStaleStakesForbiddenUpdate(log types.Log) (*ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdate, error) {
	event := new(ContractIncredibleDotProductTaskManagerStaleStakesForbiddenUpdate)
	if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "StaleStakesForbiddenUpdate", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfullyIterator is returned from FilterTaskChallengedSuccessfully and is used to iterate over the raw logs and unpacked data for TaskChallengedSuccessfully events raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfullyIterator struct {
	Event *ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfully // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfullyIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfully)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfully)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfullyIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfullyIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfully represents a TaskChallengedSuccessfully event raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfully struct {
	TaskIndex  uint32
	Challenger common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterTaskChallengedSuccessfully is a free log retrieval operation binding the contract event 0xc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec.
//
// Solidity: event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) FilterTaskChallengedSuccessfully(opts *bind.FilterOpts, taskIndex []uint32, challenger []common.Address) (*ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfullyIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var challengerRule []interface{}
	for _, challengerItem := range challenger {
		challengerRule = append(challengerRule, challengerItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.FilterLogs(opts, "TaskChallengedSuccessfully", taskIndexRule, challengerRule)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfullyIterator{contract: _ContractIncredibleDotProductTaskManager.contract, event: "TaskChallengedSuccessfully", logs: logs, sub: sub}, nil
}

// WatchTaskChallengedSuccessfully is a free log subscription operation binding the contract event 0xc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec.
//
// Solidity: event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) WatchTaskChallengedSuccessfully(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfully, taskIndex []uint32, challenger []common.Address) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var challengerRule []interface{}
	for _, challengerItem := range challenger {
		challengerRule = append(challengerRule, challengerItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.WatchLogs(opts, "TaskChallengedSuccessfully", taskIndexRule, challengerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfully)
				if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "TaskChallengedSuccessfully", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseTaskChallengedSuccessfully is a log parse operation binding the contract event 0xc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec.
//
// Solidity: event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) ParseTaskChallengedSuccessfully(log types.Log) (*ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfully, error) {
	event := new(ContractIncredibleDotProductTaskManagerTaskChallengedSuccessfully)
	if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "TaskChallengedSuccessfully", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfullyIterator is returned from FilterTaskChallengedUnsuccessfully and is used to iterate over the raw logs and unpacked data for TaskChallengedUnsuccessfully events raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfullyIterator struct {
	Event *ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfully // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfullyIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfully)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfully)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfullyIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfullyIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfully represents a TaskChallengedUnsuccessfully event raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfully struct {
	TaskIndex  uint32
	Challenger common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterTaskChallengedUnsuccessfully is a free log retrieval operation binding the contract event 0xfd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb05.
//
// Solidity: event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) FilterTaskChallengedUnsuccessfully(opts *bind.FilterOpts, taskIndex []uint32, challenger []common.Address) (*ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfullyIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var challengerRule []interface{}
	for _, challengerItem := range challenger {
		challengerRule = append(challengerRule, challengerItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.FilterLogs(opts, "TaskChallengedUnsuccessfully", taskIndexRule, challengerRule)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfullyIterator{contract: _ContractIncredibleDotProductTaskManager.contract, event: "TaskChallengedUnsuccessfully", logs: logs, sub: sub}, nil
}

// WatchTaskChallengedUnsuccessfully is a free log subscription operation binding the contract event 0xfd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb05.
//
// Solidity: event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) WatchTaskChallengedUnsuccessfully(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfully, taskIndex []uint32, challenger []common.Address) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}
	var challengerRule []interface{}
	for _, challengerItem := range challenger {
		challengerRule = append(challengerRule, challengerItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.WatchLogs(opts, "TaskChallengedUnsuccessfully", taskIndexRule, challengerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfully)
				if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "TaskChallengedUnsuccessfully", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseTaskChallengedUnsuccessfully is a log parse operation binding the contract event 0xfd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb05.
//
// Solidity: event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) ParseTaskChallengedUnsuccessfully(log types.Log) (*ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfully, error) {
	event := new(ContractIncredibleDotProductTaskManagerTaskChallengedUnsuccessfully)
	if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "TaskChallengedUnsuccessfully", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductTaskManagerTaskCompletedIterator is returned from FilterTaskCompleted and is used to iterate over the raw logs and unpacked data for TaskCompleted events raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerTaskCompletedIterator struct {
	Event *ContractIncredibleDotProductTaskManagerTaskCompleted // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractIncredibleDotProductTaskManagerTaskCompletedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductTaskManagerTaskCompleted)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractIncredibleDotProductTaskManagerTaskCompleted)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractIncredibleDotProductTaskManagerTaskCompletedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductTaskManagerTaskCompletedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductTaskManagerTaskCompleted represents a TaskCompleted event raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerTaskCompleted struct {
	TaskIndex uint32
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterTaskCompleted is a free log retrieval operation binding the contract event 0x9a144f228a931b9d0d1696fbcdaf310b24b5d2d21e799db623fc986a0f547430.
//
// Solidity: event TaskCompleted(uint32 indexed taskIndex)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) FilterTaskCompleted(opts *bind.FilterOpts, taskIndex []uint32) (*ContractIncredibleDotProductTaskManagerTaskCompletedIterator, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.FilterLogs(opts, "TaskCompleted", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerTaskCompletedIterator{contract: _ContractIncredibleDotProductTaskManager.contract, event: "TaskCompleted", logs: logs, sub: sub}, nil
}

// WatchTaskCompleted is a free log subscription operation binding the contract event 0x9a144f228a931b9d0d1696fbcdaf310b24b5d2d21e799db623fc986a0f547430.
//
// Solidity: event TaskCompleted(uint32 indexed taskIndex)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) WatchTaskCompleted(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductTaskManagerTaskCompleted, taskIndex []uint32) (event.Subscription, error) {

	var taskIndexRule []interface{}
	for _, taskIndexItem := range taskIndex {
		taskIndexRule = append(taskIndexRule, taskIndexItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.WatchLogs(opts, "TaskCompleted", taskIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductTaskManagerTaskCompleted)
				if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "TaskCompleted", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseTaskCompleted is a log parse operation binding the contract event 0x9a144f228a931b9d0d1696fbcdaf310b24b5d2d21e799db623fc986a0f547430.
//
// Solidity: event TaskCompleted(uint32 indexed taskIndex)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) ParseTaskCompleted(log types.Log) (*ContractIncredibleDotProductTaskManagerTaskCompleted, error) {
	event := new(ContractIncredibleDotProductTaskManagerTaskCompleted)
	if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "TaskCompleted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductTaskManagerTaskRespondedIterator is returned from FilterTaskResponded and is used to iterate over the raw logs and unpacked data for TaskResponded events raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerTaskRespondedIterator struct {
	Event *ContractIncredibleDotProductTaskManagerTaskResponded // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractIncredibleDotProductTaskManagerTaskRespondedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductTaskManagerTaskResponded)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractIncredibleDotProductTaskManagerTaskResponded)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractIncredibleDotProductTaskManagerTaskRespondedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductTaskManagerTaskRespondedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductTaskManagerTaskResponded represents a TaskResponded event raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerTaskResponded struct {
	TaskResponse         IIncredibleDotProductTaskManagerTaskResponse
	TaskResponseMetadata IIncredibleDotProductTaskManagerTaskResponseMetadata
	Raw                  types.Log // Blockchain specific contextual infos
}

// FilterTaskResponded is a free log retrieval operation binding the contract event 0x349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a.
//
// Solidity: event TaskResponded((uint32,uint256) taskResponse, (uint32,bytes32) taskResponseMetadata)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) FilterTaskResponded(opts *bind.FilterOpts) (*ContractIncredibleDotProductTaskManagerTaskRespondedIterator, error) {

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.FilterLogs(opts, "TaskResponded")
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerTaskRespondedIterator{contract: _ContractIncredibleDotProductTaskManager.contract, event: "TaskResponded", logs: logs, sub: sub}, nil
}

// WatchTaskResponded is a free log subscription operation binding the contract event 0x349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a.
//
// Solidity: event TaskResponded((uint32,uint256) taskResponse, (uint32,bytes32) taskResponseMetadata)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) WatchTaskResponded(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductTaskManagerTaskResponded) (event.Subscription, error) {

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.WatchLogs(opts, "TaskResponded")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductTaskManagerTaskResponded)
				if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "TaskResponded", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseTaskResponded is a log parse operation binding the contract event 0x349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a.
//
// Solidity: event TaskResponded((uint32,uint256) taskResponse, (uint32,bytes32) taskResponseMetadata)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) ParseTaskResponded(log types.Log) (*ContractIncredibleDotProductTaskManagerTaskResponded, error) {
	event := new(ContractIncredibleDotProductTaskManagerTaskResponded)
	if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "TaskResponded", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductTaskManagerUnpausedIterator is returned from FilterUnpaused and is used to iterate over the raw logs and unpacked data for Unpaused events raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerUnpausedIterator struct {
	Event *ContractIncredibleDotProductTaskManagerUnpaused // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractIncredibleDotProductTaskManagerUnpausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductTaskManagerUnpaused)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractIncredibleDotProductTaskManagerUnpaused)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractIncredibleDotProductTaskManagerUnpausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductTaskManagerUnpausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductTaskManagerUnpaused represents a Unpaused event raised by the ContractIncredibleDotProductTaskManager contract.
type ContractIncredibleDotProductTaskManagerUnpaused struct {
	Account         common.Address
	NewPausedStatus *big.Int
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterUnpaused is a free log retrieval operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) FilterUnpaused(opts *bind.FilterOpts, account []common.Address) (*ContractIncredibleDotProductTaskManagerUnpausedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.FilterLogs(opts, "Unpaused", accountRule)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductTaskManagerUnpausedIterator{contract: _ContractIncredibleDotProductTaskManager.contract, event: "Unpaused", logs: logs, sub: sub}, nil
}

// WatchUnpaused is a free log subscription operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) WatchUnpaused(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductTaskManagerUnpaused, account []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractIncredibleDotProductTaskManager.contract.WatchLogs(opts, "Unpaused", accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductTaskManagerUnpaused)
				if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "Unpaused", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseUnpaused is a log parse operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractIncredibleDotProductTaskManager *ContractIncredibleDotProductTaskManagerFilterer) ParseUnpaused(log types.Log) (*ContractIncredibleDotProductTaskManagerUnpaused, error) {
	event := new(ContractIncredibleDotProductTaskManagerUnpaused)
	if err := _ContractIncredibleDotProductTaskManager.contract.UnpackLog(event, "Unpaused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
