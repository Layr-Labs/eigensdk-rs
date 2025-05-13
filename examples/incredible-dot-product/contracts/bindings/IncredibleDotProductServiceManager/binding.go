// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package contractIncredibleDotProductServiceManager

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

// IRewardsCoordinatorTypesOperatorDirectedRewardsSubmission is an auto generated low-level Go binding around an user-defined struct.
type IRewardsCoordinatorTypesOperatorDirectedRewardsSubmission struct {
	StrategiesAndMultipliers []IRewardsCoordinatorTypesStrategyAndMultiplier
	Token                    common.Address
	OperatorRewards          []IRewardsCoordinatorTypesOperatorReward
	StartTimestamp           uint32
	Duration                 uint32
	Description              string
}

// IRewardsCoordinatorTypesOperatorReward is an auto generated low-level Go binding around an user-defined struct.
type IRewardsCoordinatorTypesOperatorReward struct {
	Operator common.Address
	Amount   *big.Int
}

// IRewardsCoordinatorTypesRewardsSubmission is an auto generated low-level Go binding around an user-defined struct.
type IRewardsCoordinatorTypesRewardsSubmission struct {
	StrategiesAndMultipliers []IRewardsCoordinatorTypesStrategyAndMultiplier
	Token                    common.Address
	Amount                   *big.Int
	StartTimestamp           uint32
	Duration                 uint32
}

// IRewardsCoordinatorTypesStrategyAndMultiplier is an auto generated low-level Go binding around an user-defined struct.
type IRewardsCoordinatorTypesStrategyAndMultiplier struct {
	Strategy   common.Address
	Multiplier *big.Int
}

// ISignatureUtilsMixinTypesSignatureWithSaltAndExpiry is an auto generated low-level Go binding around an user-defined struct.
type ISignatureUtilsMixinTypesSignatureWithSaltAndExpiry struct {
	Signature []byte
	Salt      [32]byte
	Expiry    *big.Int
}

// ContractIncredibleDotProductServiceManagerMetaData contains all meta data concerning the ContractIncredibleDotProductServiceManager contract.
var ContractIncredibleDotProductServiceManagerMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"_avsDirectory\",\"type\":\"address\",\"internalType\":\"contractIAVSDirectory\"},{\"name\":\"_registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractISlashingRegistryCoordinator\"},{\"name\":\"_stakeRegistry\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"},{\"name\":\"rewards_coordinator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"allocationManager\",\"type\":\"address\",\"internalType\":\"contractIAllocationManager\"},{\"name\":\"_permissionController\",\"type\":\"address\",\"internalType\":\"contractIPermissionController\"},{\"name\":\"_incredibleDotProductTaskManager\",\"type\":\"address\",\"internalType\":\"contractIIncredibleDotProductTaskManager\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"addPendingAdmin\",\"inputs\":[{\"name\":\"admin\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"avsDirectory\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createAVSRewardsSubmission\",\"inputs\":[{\"name\":\"rewardsSubmissions\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinatorTypes.RewardsSubmission[]\",\"components\":[{\"name\":\"strategiesAndMultipliers\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinatorTypes.StrategyAndMultiplier[]\",\"components\":[{\"name\":\"strategy\",\"type\":\"address\",\"internalType\":\"contractIStrategy\"},{\"name\":\"multiplier\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"token\",\"type\":\"address\",\"internalType\":\"contractIERC20\"},{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"startTimestamp\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"duration\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"createOperatorDirectedAVSRewardsSubmission\",\"inputs\":[{\"name\":\"operatorDirectedRewardsSubmissions\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[]\",\"components\":[{\"name\":\"strategiesAndMultipliers\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinatorTypes.StrategyAndMultiplier[]\",\"components\":[{\"name\":\"strategy\",\"type\":\"address\",\"internalType\":\"contractIStrategy\"},{\"name\":\"multiplier\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"token\",\"type\":\"address\",\"internalType\":\"contractIERC20\"},{\"name\":\"operatorRewards\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinatorTypes.OperatorReward[]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"startTimestamp\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"duration\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"description\",\"type\":\"string\",\"internalType\":\"string\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"deregisterOperatorFromAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"deregisterOperatorFromOperatorSets\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorSetIds\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"getOperatorRestakedStrategies\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getRestakeableStrategies\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"incredibleDotProductTaskManager\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIIncredibleDotProductTaskManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"rewardsInitiator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registerOperatorToAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorSignature\",\"type\":\"tuple\",\"internalType\":\"structISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry\",\"components\":[{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"expiry\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"removeAdmin\",\"inputs\":[{\"name\":\"admin\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"removeAppointee\",\"inputs\":[{\"name\":\"appointee\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"target\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"selector\",\"type\":\"bytes4\",\"internalType\":\"bytes4\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"removePendingAdmin\",\"inputs\":[{\"name\":\"pendingAdmin\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rewardsInitiator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setAppointee\",\"inputs\":[{\"name\":\"appointee\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"target\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"selector\",\"type\":\"bytes4\",\"internalType\":\"bytes4\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setClaimerFor\",\"inputs\":[{\"name\":\"claimer\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setRewardsInitiator\",\"inputs\":[{\"name\":\"newRewardsInitiator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateAVSMetadataURI\",\"inputs\":[{\"name\":\"_metadataURI\",\"type\":\"string\",\"internalType\":\"string\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RewardsInitiatorUpdated\",\"inputs\":[{\"name\":\"prevRewardsInitiator\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"newRewardsInitiator\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"DelayPeriodNotPassed\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"OnlyRegistryCoordinator\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"OnlyRewardsInitiator\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"OnlyStakeRegistry\",\"inputs\":[]}]",
	Bin: "0x6101603461024957601f6122f338819003918201601f19168301916001600160401b0383118484101761024d5780849260e094604052833981010312610249578051906001600160a01b0382168203610249576020810151906001600160a01b03821682036102495760408101516001600160a01b03811681036102495760608201516001600160a01b0381169390849003610249576080830151936001600160a01b03851685036102495760a0840151936001600160a01b03851685036102495760c00151956001600160a01b03871687036102495760805260c05260e052610100526101205260a0525f5460ff8160081c166101f45760ff808216036101ba575b5061014052604051612091908161026282396080518181816106940152818161078801528181610ca50152610de3015260a051816104b9015260c0518181816101490152818161082d0152610aec015260e0518181816104600152818161075601528181610c78015281816114b00152611a0d0152610100518181816115ca0152611a590152610120518181816105c001528181610b7901528181611026015281816110a501526111430152610140518161070b0152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f610102565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f5f3560e01c80631785f53c1461111f5780631fdb0cfd14611088578063279432eb1461100157806333cfb7b714610fdc5780633bc28c8c14610fb7578063485cc95514610e125780636b3aa72e14610dcd578063715018a614610d705780638da5cb5b14610d475780639926ee7d14610bdb5780639da16d8e14610b54578063a0169ddd14610ac7578063a20b99bf146107f0578063a364f4da1461073a578063a758ea4d146106f5578063a98fb35514610647578063ba550880146105a3578063c1a8e2c5146103e2578063e481af9d146103b6578063f2fde38b14610325578063fc299dee146102fc5763fce36c7d1461010c575f80fd5b346101e95760203660031901126101e9576004356001600160401b0381116102f85761013c903690600401611326565b610147929192611d7e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690825b8181106102945750813b15610290576040519363fce36c7d60e01b8552816024860160206004880152526044850160448360051b87010192828690609e19813603015b8383106101f75788808b8181808c0381838f5af180156101ec576101d85750f35b816101e2916112b4565b6101e95780f35b80fd5b6040513d84823e3d90fd5b9091929394956043198a820301865286358281121561028c5760206001928582930190608063ffffffff61027a826102406102328780611966565b60a0885260a088019161199a565b95898060a01b036102528983016111e4565b1688870152604081013560408701528361026e6060830161136d565b1660608701520161136d565b169101529801960194930191906101b7565b8980fd5b8280fd5b806102c96102b060206102aa600195878b611c5e565b01611952565b60406102bd84878b611c5e565b01359030903390611da1565b6102f26102dc60206102aa84878b611c5e565b8560406102ea85888c611c5e565b013591611dec565b01610174565b5080fd5b50346101e957806003193601126101e9576065546040516001600160a01b039091168152602090f35b50346101e95760203660031901126101e95761033f6111ce565b610347611c80565b6001600160a01b038116156103625761035f90611d36565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b50346101e957806003193601126101e9576103de6103d26119fe565b60405191829182611243565b0390f35b50346101e95760403660031901126101e9576103fc6111ce565b90602435916001600160401b0383116102f857366023840112156102f85782600401359261042984611356565b9361043760405195866112b4565b8085526024602086019160051b8301019136831161059f57602401905b828210610587575050507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303610578576040519061049b82611285565b6001600160a01b0390811682523060208301908152604083019485527f000000000000000000000000000000000000000000000000000000000000000090911693909190843b1561057457604051636e3492b560e01b815260206004820181905292516001600160a01b03908116602483015293519093166044840152516060606484015280516084840181905260a48401929190910190845b81811061055857505050818394818581819503925af180156101ec576101d85750f35b825163ffffffff16845260209384019390920191600101610535565b8380fd5b634394dbdf60e11b8252600482fd5b602080916105948461136d565b815201910190610454565b8480fd5b50346101e957806105b3366111f8565b6105be929192611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561059f57604051630664120160e01b81523060048201526001600160a01b0393841660248201529390921660448401526001600160e01b031916606483015282908290818381608481015b03925af180156101ec576101d85750f35b50346101e95760203660031901126101e957806004356001600160401b0381116106f257366023820112156106f25761068a9036906024816004013591016112f0565b610692611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156106f05760405163a98fb35560e01b81526020600482015291839183918290849082906106369060248301906118d7565b505b50fd5b50346101e957806003193601126101e9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b50346101e95760203660031901126101e9576107546111ce565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036105785781907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156106f0576040516351b27a6d60e11b81526001600160a01b0390911660048201529082908290602490829084905af180156101ec576101d85750f35b50346101e95760203660031901126101e9576004356001600160401b0381116102f857610821903690600401611326565b919061082b611d7e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316825b848110610a125750803b156102905760409391935191634e5cd2fd60e11b8352806044840130600486015260406024860152526064830160648260051b850101918691869760be19813603015b838a106108c5578880898181808b0381838e5af180156101ec576101d85750f35b909192939460631988820301845285358281121561028c578301906108fb6108ed8380611966565b60c0845260c084019161199a565b916001600160a01b03610910602083016111e4565b16602083015260206109256040830183611966565b848603604086015280865294909101938c905b8082106109e25750505063ffffffff6109536060830161136d565b16606083015263ffffffff61096a6080830161136d565b16608083015260a0810135601e19823603018112156109de570190602082359201906001600160401b0383116109de5782360382136109de57838360209485948460a0879660019a03910152818452848401378d838284010152601f801991011601019701940199019891909392936108a4565b8b80fd5b909194604080600192838060a01b036109fa8a6111e4565b16815260208901356020820152019601920190610938565b90929193829483955b610a33610a298585856118fb565b604081019061191d565b9050871015610a8357610a4a610a298585856118fb565b881015610a6f576001916020610a67928a60061b010135906113ee565b960195610a1b565b634e487b7160e01b86526032600452602486fd5b919550929391600191610ac190610aab813033610aa660206102aa898f8e6118fb565b611da1565b84610abc60206102aa868c8b6118fb565b611dec565b01610857565b50346101e95760203660031901126101e95780610ae26111ce565b610aea611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156106f05760405163a0169ddd60e01b81526001600160a01b0390911660048201529082908290602490829084905af180156101ec576101d85750f35b50346101e95760203660031901126101e95780610b6f6111ce565b610b77611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156106f057604051634f906cf960e01b81523060048201526001600160a01b0390921660248301528290829081838160448101610636565b50346101e95760403660031901126101e957610bf56111ce565b906024356001600160401b0381116102f857606060031982360301126102f85760405192610c2284611285565b81600401356001600160401b0381116105745782013660238201121561057457610c569036906024600482013591016112f0565b8452602084019060248301358252604460408601930135835260018060a01b037f0000000000000000000000000000000000000000000000000000000000000000163303610d385792938493907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610d345785610d1693819560405197889687958694639926ee7d60e01b865260018060a01b0316600486015260406024860152516060604486015260a48501906118d7565b9151606484015251608483015203925af180156101ec576101d85750f35b8580fd5b634394dbdf60e11b8452600484fd5b50346101e957806003193601126101e9576033546040516001600160a01b039091168152602090f35b50346101e957806003193601126101e957610d89611c80565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b50346101e957806003193601126101e9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b50346101e95760403660031901126101e957610e2c6111ce565b602435906001600160a01b03821682036102905782549160ff8360081c161592838094610faa575b8015610f93575b15610f375760ff198116600117855583610f26575b5060ff845460081c1615610ecd57610e8a610e8f92611d36565b611cd8565b610e965780f35b61ff001981541681557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a180f35b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b61ffff19166101011784555f610e70565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b158015610e5b5750600160ff821614610e5b565b50600160ff821610610e54565b50346101e95760203660031901126101e95761035f610fd46111ce565b610e8a611c80565b50346101e95760203660031901126101e9576103de6103d2610ffc6111ce565b611490565b50346101e95760203660031901126101e9578061101c6111ce565b611024611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156106f05760405163eb5a4e8760e01b81523060048201526001600160a01b0390921660248301528290829081838160448101610636565b50346101e95780611098366111f8565b6110a3929192611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561059f57604051634a86c03760e11b81523060048201526001600160a01b0393841660248201529390921660448401526001600160e01b03191660648301528290829081838160848101610636565b50346111ca5760203660031901126111ca576111396111ce565b611141611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156111ca5760405163268959e560e01b81523060048201526001600160a01b039290921660248301525f908290604490829084905af180156111bf576111b1575080f35b6111bd91505f906112b4565b005b6040513d5f823e3d90fd5b5f80fd5b600435906001600160a01b03821682036111ca57565b35906001600160a01b03821682036111ca57565b60609060031901126111ca576004356001600160a01b03811681036111ca57906024356001600160a01b03811681036111ca57906044356001600160e01b0319811681036111ca5790565b60206040818301928281528451809452019201905f5b8181106112665750505090565b82516001600160a01b0316845260209384019390920191600101611259565b606081019081106001600160401b038211176112a057604052565b634e487b7160e01b5f52604160045260245ffd5b90601f801991011681019081106001600160401b038211176112a057604052565b6001600160401b0381116112a057601f01601f191660200190565b9291926112fc826112d5565b9161130a60405193846112b4565b8294818452818301116111ca578281602093845f960137010152565b9181601f840112156111ca578235916001600160401b0383116111ca576020808501948460051b0101116111ca57565b6001600160401b0381116112a05760051b60200190565b359063ffffffff821682036111ca57565b908160209103126111ca575160ff811681036111ca5790565b906113a182611356565b6113ae60405191826112b4565b82815280926113bf601f1991611356565b0190602036910137565b9081518110156113da570160200190565b634e487b7160e01b5f52603260045260245ffd5b919082018092116113fb57565b634e487b7160e01b5f52601160045260245ffd5b908160409103126111ca5760405190604082018281106001600160401b038211176112a0576040528051906001600160a01b03821682036111ca57602091835201516bffffffffffffffffffffffff811681036111ca57602082015290565b80518210156113da5760209160051b010190565b5f1981146113fb5760010190565b6040516309aa152760e11b81526001600160a01b039182166004820152907f000000000000000000000000000000000000000000000000000000000000000016602082602481845afa9182156111bf575f926118a3575b506040519163871ef04960e01b83526004830152602082602481845afa9182156111bf575f9261185f575b506001600160c01b0382169081159081156117fc575b506117e257805f915b6117bd575061ffff16611543816112d5565b9061155160405192836112b4565b808252611560601f19916112d5565b013660208301375f5f5b82518210806117b2575b156115c2576001811b84166001600160c01b031661159b575b61159690611482565b61156a565b9060016115969160ff60f81b8460f81b165f1a6115b882876113c9565b530191905061158d565b505f939250507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b815185101561167d5761160885836113c9565b5160f81c60405190633ca5a5f560e01b82526004820152602081602481875afa9081156111bf575f9161164b575b50611643906001926113ee565b9401936115f5565b90506020813d8211611675575b81611665602093836112b4565b810103126111ca57516001611636565b3d9150611658565b611688919450611397565b925f905f5b81518110156117ac576116a081836113c9565b5160f81c60405190633ca5a5f560e01b8252806004830152602082602481895afa9182156111bf575f92611779575b50905f915b8183106116e65750505060010161168d565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156111bf57600192611742925f9161174b575b50838060a01b0390511661173c828d61146e565b52611482565b950191906116d4565b61176c915060403d8111611772575b61176481836112b4565b81019061140f565b5f611728565b503d61175a565b9091506020813d82116117a4575b81611794602093836112b4565b810103126111ca5751905f6116cf565b3d9150611787565b50505050565b506101008110611574565b5f1981018181116113fb5761ffff9116911661ffff81146113fb576001019080611531565b50506040516117f26020826112b4565b5f81525f36813790565b604051639aa1653d60e01b81529150602090829060049082905afa80156111bf5760ff915f91611830575b5016155f611528565b611852915060203d602011611858575b61184a81836112b4565b81019061137e565b5f611827565b503d611840565b9091506020813d60201161189b575b8161187b602093836112b4565b810103126111ca57516001600160c01b03811681036111ca57905f611512565b3d915061186e565b9091506020813d6020116118cf575b816118bf602093836112b4565b810103126111ca5751905f6114e7565b3d91506118b2565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91908110156113da5760051b8101359060be19813603018212156111ca570190565b903590601e19813603018212156111ca57018035906001600160401b0382116111ca57602001918160061b360383136111ca57565b356001600160a01b03811681036111ca5790565b9035601e19823603018112156111ca5701602081359101916001600160401b0382116111ca578160061b360383136111ca57565b916020908281520191905f905b8082106119b45750505090565b909192833560018060a01b0381168091036111ca5781526020840135906bffffffffffffffffffffffff82168092036111ca576040816001936020839401520194019201906119a7565b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156111bf5760ff915f91611c3f575b501680156117e2577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b808310611bc95750611a999150611397565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156111bf5760ff915f91611bab575b5016811015611ba457604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa9182156111bf575f92611b71575b50905f915b818310611b1357505050600101611a9e565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156111bf57600192611b68925f9161174b5750838060a01b0390511661173c828d61146e565b95019190611b01565b9091506020813d8211611b9c575b81611b8c602093836112b4565b810103126111ca5751905f611afc565b3d9150611b7f565b5092505050565b611bc3915060203d81116118585761184a81836112b4565b5f611ac4565b90604051633ca5a5f560e01b815260ff84166004820152602081602481885afa9081156111bf575f91611c0d575b50611c04906001926113ee565b92019190611a87565b90506020813d8211611c37575b81611c27602093836112b4565b810103126111ca57516001611bf7565b3d9150611c1a565b611c58915060203d6020116118585761184a81836112b4565b5f611a4f565b91908110156113da5760051b81013590609e19813603018212156111ca570190565b6033546001600160a01b03163303611c9457565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6065546001600160a01b03163303611d9257565b638e79fdb560e01b5f5260045ffd5b6040516323b872dd60e01b60208201526001600160a01b039283166024820152929091166044830152606480830193909352918152611dea91611de56084836112b4565b611eaa565b565b604051636eb1769f60e11b81523060048201526001600160a01b0383166024820152602081806044810103816001600160a01b0386165afa9081156111bf575f91611e76575b50611dea93611e40916113ee565b60405163095ea7b360e01b60208201526001600160a01b0390931660248401526044808401919091528252611de56064836112b4565b90506020813d602011611ea2575b81611e91602093836112b4565b810103126111ca5751611dea611e32565b3d9150611e84565b90611f299160018060a01b03165f8060405193611ec86040866112b4565b602085527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564602086015260208151910182855af13d15611fba573d91611f0d836112d5565b92611f1b60405194856112b4565b83523d5f602085013e611fbe565b8051908115918215611f97575b505015611f3f57565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b81925090602091810103126111ca576020015180151581036111ca575f80611f36565b6060915b919290156120205750815115611fd2575090565b3b15611fdb5790565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b8251909150156120335750805190602001fd5b60405162461bcd60e51b8152602060048201529081906120579060248301906118d7565b0390fdfea26469706673582212202a21b7ab19baac69d5fe0c6c4369dbcfadd266cc9c7b2e4b8c20e5380fceed3e64736f6c634300081b0033",
}

// ContractIncredibleDotProductServiceManagerABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractIncredibleDotProductServiceManagerMetaData.ABI instead.
var ContractIncredibleDotProductServiceManagerABI = ContractIncredibleDotProductServiceManagerMetaData.ABI

// ContractIncredibleDotProductServiceManagerBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractIncredibleDotProductServiceManagerMetaData.Bin instead.
var ContractIncredibleDotProductServiceManagerBin = ContractIncredibleDotProductServiceManagerMetaData.Bin

// DeployContractIncredibleDotProductServiceManager deploys a new Ethereum contract, binding an instance of ContractIncredibleDotProductServiceManager to it.
func DeployContractIncredibleDotProductServiceManager(auth *bind.TransactOpts, backend bind.ContractBackend, _avsDirectory common.Address, _registryCoordinator common.Address, _stakeRegistry common.Address, rewards_coordinator common.Address, allocationManager common.Address, _permissionController common.Address, _incredibleDotProductTaskManager common.Address) (common.Address, *types.Transaction, *ContractIncredibleDotProductServiceManager, error) {
	parsed, err := ContractIncredibleDotProductServiceManagerMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractIncredibleDotProductServiceManagerBin), backend, _avsDirectory, _registryCoordinator, _stakeRegistry, rewards_coordinator, allocationManager, _permissionController, _incredibleDotProductTaskManager)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &ContractIncredibleDotProductServiceManager{ContractIncredibleDotProductServiceManagerCaller: ContractIncredibleDotProductServiceManagerCaller{contract: contract}, ContractIncredibleDotProductServiceManagerTransactor: ContractIncredibleDotProductServiceManagerTransactor{contract: contract}, ContractIncredibleDotProductServiceManagerFilterer: ContractIncredibleDotProductServiceManagerFilterer{contract: contract}}, nil
}

// ContractIncredibleDotProductServiceManager is an auto generated Go binding around an Ethereum contract.
type ContractIncredibleDotProductServiceManager struct {
	ContractIncredibleDotProductServiceManagerCaller     // Read-only binding to the contract
	ContractIncredibleDotProductServiceManagerTransactor // Write-only binding to the contract
	ContractIncredibleDotProductServiceManagerFilterer   // Log filterer for contract events
}

// ContractIncredibleDotProductServiceManagerCaller is an auto generated read-only Go binding around an Ethereum contract.
type ContractIncredibleDotProductServiceManagerCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractIncredibleDotProductServiceManagerTransactor is an auto generated write-only Go binding around an Ethereum contract.
type ContractIncredibleDotProductServiceManagerTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractIncredibleDotProductServiceManagerFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type ContractIncredibleDotProductServiceManagerFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractIncredibleDotProductServiceManagerSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type ContractIncredibleDotProductServiceManagerSession struct {
	Contract     *ContractIncredibleDotProductServiceManager // Generic contract binding to set the session for
	CallOpts     bind.CallOpts                               // Call options to use throughout this session
	TransactOpts bind.TransactOpts                           // Transaction auth options to use throughout this session
}

// ContractIncredibleDotProductServiceManagerCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type ContractIncredibleDotProductServiceManagerCallerSession struct {
	Contract *ContractIncredibleDotProductServiceManagerCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                                     // Call options to use throughout this session
}

// ContractIncredibleDotProductServiceManagerTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type ContractIncredibleDotProductServiceManagerTransactorSession struct {
	Contract     *ContractIncredibleDotProductServiceManagerTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                                     // Transaction auth options to use throughout this session
}

// ContractIncredibleDotProductServiceManagerRaw is an auto generated low-level Go binding around an Ethereum contract.
type ContractIncredibleDotProductServiceManagerRaw struct {
	Contract *ContractIncredibleDotProductServiceManager // Generic contract binding to access the raw methods on
}

// ContractIncredibleDotProductServiceManagerCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type ContractIncredibleDotProductServiceManagerCallerRaw struct {
	Contract *ContractIncredibleDotProductServiceManagerCaller // Generic read-only contract binding to access the raw methods on
}

// ContractIncredibleDotProductServiceManagerTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type ContractIncredibleDotProductServiceManagerTransactorRaw struct {
	Contract *ContractIncredibleDotProductServiceManagerTransactor // Generic write-only contract binding to access the raw methods on
}

// NewContractIncredibleDotProductServiceManager creates a new instance of ContractIncredibleDotProductServiceManager, bound to a specific deployed contract.
func NewContractIncredibleDotProductServiceManager(address common.Address, backend bind.ContractBackend) (*ContractIncredibleDotProductServiceManager, error) {
	contract, err := bindContractIncredibleDotProductServiceManager(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductServiceManager{ContractIncredibleDotProductServiceManagerCaller: ContractIncredibleDotProductServiceManagerCaller{contract: contract}, ContractIncredibleDotProductServiceManagerTransactor: ContractIncredibleDotProductServiceManagerTransactor{contract: contract}, ContractIncredibleDotProductServiceManagerFilterer: ContractIncredibleDotProductServiceManagerFilterer{contract: contract}}, nil
}

// NewContractIncredibleDotProductServiceManagerCaller creates a new read-only instance of ContractIncredibleDotProductServiceManager, bound to a specific deployed contract.
func NewContractIncredibleDotProductServiceManagerCaller(address common.Address, caller bind.ContractCaller) (*ContractIncredibleDotProductServiceManagerCaller, error) {
	contract, err := bindContractIncredibleDotProductServiceManager(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductServiceManagerCaller{contract: contract}, nil
}

// NewContractIncredibleDotProductServiceManagerTransactor creates a new write-only instance of ContractIncredibleDotProductServiceManager, bound to a specific deployed contract.
func NewContractIncredibleDotProductServiceManagerTransactor(address common.Address, transactor bind.ContractTransactor) (*ContractIncredibleDotProductServiceManagerTransactor, error) {
	contract, err := bindContractIncredibleDotProductServiceManager(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductServiceManagerTransactor{contract: contract}, nil
}

// NewContractIncredibleDotProductServiceManagerFilterer creates a new log filterer instance of ContractIncredibleDotProductServiceManager, bound to a specific deployed contract.
func NewContractIncredibleDotProductServiceManagerFilterer(address common.Address, filterer bind.ContractFilterer) (*ContractIncredibleDotProductServiceManagerFilterer, error) {
	contract, err := bindContractIncredibleDotProductServiceManager(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductServiceManagerFilterer{contract: contract}, nil
}

// bindContractIncredibleDotProductServiceManager binds a generic wrapper to an already deployed contract.
func bindContractIncredibleDotProductServiceManager(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := ContractIncredibleDotProductServiceManagerMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractIncredibleDotProductServiceManager.Contract.ContractIncredibleDotProductServiceManagerCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.ContractIncredibleDotProductServiceManagerTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.ContractIncredibleDotProductServiceManagerTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractIncredibleDotProductServiceManager.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.contract.Transact(opts, method, params...)
}

// AvsDirectory is a free data retrieval call binding the contract method 0x6b3aa72e.
//
// Solidity: function avsDirectory() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCaller) AvsDirectory(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductServiceManager.contract.Call(opts, &out, "avsDirectory")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// AvsDirectory is a free data retrieval call binding the contract method 0x6b3aa72e.
//
// Solidity: function avsDirectory() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) AvsDirectory() (common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.AvsDirectory(&_ContractIncredibleDotProductServiceManager.CallOpts)
}

// AvsDirectory is a free data retrieval call binding the contract method 0x6b3aa72e.
//
// Solidity: function avsDirectory() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCallerSession) AvsDirectory() (common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.AvsDirectory(&_ContractIncredibleDotProductServiceManager.CallOpts)
}

// GetOperatorRestakedStrategies is a free data retrieval call binding the contract method 0x33cfb7b7.
//
// Solidity: function getOperatorRestakedStrategies(address operator) view returns(address[])
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCaller) GetOperatorRestakedStrategies(opts *bind.CallOpts, operator common.Address) ([]common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductServiceManager.contract.Call(opts, &out, "getOperatorRestakedStrategies", operator)

	if err != nil {
		return *new([]common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new([]common.Address)).(*[]common.Address)

	return out0, err

}

// GetOperatorRestakedStrategies is a free data retrieval call binding the contract method 0x33cfb7b7.
//
// Solidity: function getOperatorRestakedStrategies(address operator) view returns(address[])
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) GetOperatorRestakedStrategies(operator common.Address) ([]common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.GetOperatorRestakedStrategies(&_ContractIncredibleDotProductServiceManager.CallOpts, operator)
}

// GetOperatorRestakedStrategies is a free data retrieval call binding the contract method 0x33cfb7b7.
//
// Solidity: function getOperatorRestakedStrategies(address operator) view returns(address[])
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCallerSession) GetOperatorRestakedStrategies(operator common.Address) ([]common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.GetOperatorRestakedStrategies(&_ContractIncredibleDotProductServiceManager.CallOpts, operator)
}

// GetRestakeableStrategies is a free data retrieval call binding the contract method 0xe481af9d.
//
// Solidity: function getRestakeableStrategies() view returns(address[])
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCaller) GetRestakeableStrategies(opts *bind.CallOpts) ([]common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductServiceManager.contract.Call(opts, &out, "getRestakeableStrategies")

	if err != nil {
		return *new([]common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new([]common.Address)).(*[]common.Address)

	return out0, err

}

// GetRestakeableStrategies is a free data retrieval call binding the contract method 0xe481af9d.
//
// Solidity: function getRestakeableStrategies() view returns(address[])
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) GetRestakeableStrategies() ([]common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.GetRestakeableStrategies(&_ContractIncredibleDotProductServiceManager.CallOpts)
}

// GetRestakeableStrategies is a free data retrieval call binding the contract method 0xe481af9d.
//
// Solidity: function getRestakeableStrategies() view returns(address[])
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCallerSession) GetRestakeableStrategies() ([]common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.GetRestakeableStrategies(&_ContractIncredibleDotProductServiceManager.CallOpts)
}

// IncredibleDotProductTaskManager is a free data retrieval call binding the contract method 0xa758ea4d.
//
// Solidity: function incredibleDotProductTaskManager() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCaller) IncredibleDotProductTaskManager(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductServiceManager.contract.Call(opts, &out, "incredibleDotProductTaskManager")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// IncredibleDotProductTaskManager is a free data retrieval call binding the contract method 0xa758ea4d.
//
// Solidity: function incredibleDotProductTaskManager() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) IncredibleDotProductTaskManager() (common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.IncredibleDotProductTaskManager(&_ContractIncredibleDotProductServiceManager.CallOpts)
}

// IncredibleDotProductTaskManager is a free data retrieval call binding the contract method 0xa758ea4d.
//
// Solidity: function incredibleDotProductTaskManager() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCallerSession) IncredibleDotProductTaskManager() (common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.IncredibleDotProductTaskManager(&_ContractIncredibleDotProductServiceManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductServiceManager.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) Owner() (common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.Owner(&_ContractIncredibleDotProductServiceManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCallerSession) Owner() (common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.Owner(&_ContractIncredibleDotProductServiceManager.CallOpts)
}

// RewardsInitiator is a free data retrieval call binding the contract method 0xfc299dee.
//
// Solidity: function rewardsInitiator() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCaller) RewardsInitiator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractIncredibleDotProductServiceManager.contract.Call(opts, &out, "rewardsInitiator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// RewardsInitiator is a free data retrieval call binding the contract method 0xfc299dee.
//
// Solidity: function rewardsInitiator() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) RewardsInitiator() (common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RewardsInitiator(&_ContractIncredibleDotProductServiceManager.CallOpts)
}

// RewardsInitiator is a free data retrieval call binding the contract method 0xfc299dee.
//
// Solidity: function rewardsInitiator() view returns(address)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerCallerSession) RewardsInitiator() (common.Address, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RewardsInitiator(&_ContractIncredibleDotProductServiceManager.CallOpts)
}

// AddPendingAdmin is a paid mutator transaction binding the contract method 0x279432eb.
//
// Solidity: function addPendingAdmin(address admin) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) AddPendingAdmin(opts *bind.TransactOpts, admin common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "addPendingAdmin", admin)
}

// AddPendingAdmin is a paid mutator transaction binding the contract method 0x279432eb.
//
// Solidity: function addPendingAdmin(address admin) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) AddPendingAdmin(admin common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.AddPendingAdmin(&_ContractIncredibleDotProductServiceManager.TransactOpts, admin)
}

// AddPendingAdmin is a paid mutator transaction binding the contract method 0x279432eb.
//
// Solidity: function addPendingAdmin(address admin) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) AddPendingAdmin(admin common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.AddPendingAdmin(&_ContractIncredibleDotProductServiceManager.TransactOpts, admin)
}

// CreateAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xfce36c7d.
//
// Solidity: function createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[] rewardsSubmissions) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) CreateAVSRewardsSubmission(opts *bind.TransactOpts, rewardsSubmissions []IRewardsCoordinatorTypesRewardsSubmission) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "createAVSRewardsSubmission", rewardsSubmissions)
}

// CreateAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xfce36c7d.
//
// Solidity: function createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[] rewardsSubmissions) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) CreateAVSRewardsSubmission(rewardsSubmissions []IRewardsCoordinatorTypesRewardsSubmission) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.CreateAVSRewardsSubmission(&_ContractIncredibleDotProductServiceManager.TransactOpts, rewardsSubmissions)
}

// CreateAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xfce36c7d.
//
// Solidity: function createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[] rewardsSubmissions) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) CreateAVSRewardsSubmission(rewardsSubmissions []IRewardsCoordinatorTypesRewardsSubmission) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.CreateAVSRewardsSubmission(&_ContractIncredibleDotProductServiceManager.TransactOpts, rewardsSubmissions)
}

// CreateOperatorDirectedAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xa20b99bf.
//
// Solidity: function createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[] operatorDirectedRewardsSubmissions) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) CreateOperatorDirectedAVSRewardsSubmission(opts *bind.TransactOpts, operatorDirectedRewardsSubmissions []IRewardsCoordinatorTypesOperatorDirectedRewardsSubmission) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "createOperatorDirectedAVSRewardsSubmission", operatorDirectedRewardsSubmissions)
}

// CreateOperatorDirectedAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xa20b99bf.
//
// Solidity: function createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[] operatorDirectedRewardsSubmissions) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) CreateOperatorDirectedAVSRewardsSubmission(operatorDirectedRewardsSubmissions []IRewardsCoordinatorTypesOperatorDirectedRewardsSubmission) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.CreateOperatorDirectedAVSRewardsSubmission(&_ContractIncredibleDotProductServiceManager.TransactOpts, operatorDirectedRewardsSubmissions)
}

// CreateOperatorDirectedAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xa20b99bf.
//
// Solidity: function createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[] operatorDirectedRewardsSubmissions) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) CreateOperatorDirectedAVSRewardsSubmission(operatorDirectedRewardsSubmissions []IRewardsCoordinatorTypesOperatorDirectedRewardsSubmission) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.CreateOperatorDirectedAVSRewardsSubmission(&_ContractIncredibleDotProductServiceManager.TransactOpts, operatorDirectedRewardsSubmissions)
}

// DeregisterOperatorFromAVS is a paid mutator transaction binding the contract method 0xa364f4da.
//
// Solidity: function deregisterOperatorFromAVS(address operator) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) DeregisterOperatorFromAVS(opts *bind.TransactOpts, operator common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "deregisterOperatorFromAVS", operator)
}

// DeregisterOperatorFromAVS is a paid mutator transaction binding the contract method 0xa364f4da.
//
// Solidity: function deregisterOperatorFromAVS(address operator) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) DeregisterOperatorFromAVS(operator common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.DeregisterOperatorFromAVS(&_ContractIncredibleDotProductServiceManager.TransactOpts, operator)
}

// DeregisterOperatorFromAVS is a paid mutator transaction binding the contract method 0xa364f4da.
//
// Solidity: function deregisterOperatorFromAVS(address operator) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) DeregisterOperatorFromAVS(operator common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.DeregisterOperatorFromAVS(&_ContractIncredibleDotProductServiceManager.TransactOpts, operator)
}

// DeregisterOperatorFromOperatorSets is a paid mutator transaction binding the contract method 0xc1a8e2c5.
//
// Solidity: function deregisterOperatorFromOperatorSets(address operator, uint32[] operatorSetIds) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) DeregisterOperatorFromOperatorSets(opts *bind.TransactOpts, operator common.Address, operatorSetIds []uint32) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "deregisterOperatorFromOperatorSets", operator, operatorSetIds)
}

// DeregisterOperatorFromOperatorSets is a paid mutator transaction binding the contract method 0xc1a8e2c5.
//
// Solidity: function deregisterOperatorFromOperatorSets(address operator, uint32[] operatorSetIds) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) DeregisterOperatorFromOperatorSets(operator common.Address, operatorSetIds []uint32) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.DeregisterOperatorFromOperatorSets(&_ContractIncredibleDotProductServiceManager.TransactOpts, operator, operatorSetIds)
}

// DeregisterOperatorFromOperatorSets is a paid mutator transaction binding the contract method 0xc1a8e2c5.
//
// Solidity: function deregisterOperatorFromOperatorSets(address operator, uint32[] operatorSetIds) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) DeregisterOperatorFromOperatorSets(operator common.Address, operatorSetIds []uint32) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.DeregisterOperatorFromOperatorSets(&_ContractIncredibleDotProductServiceManager.TransactOpts, operator, operatorSetIds)
}

// Initialize is a paid mutator transaction binding the contract method 0x485cc955.
//
// Solidity: function initialize(address initialOwner, address rewardsInitiator) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) Initialize(opts *bind.TransactOpts, initialOwner common.Address, rewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "initialize", initialOwner, rewardsInitiator)
}

// Initialize is a paid mutator transaction binding the contract method 0x485cc955.
//
// Solidity: function initialize(address initialOwner, address rewardsInitiator) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) Initialize(initialOwner common.Address, rewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.Initialize(&_ContractIncredibleDotProductServiceManager.TransactOpts, initialOwner, rewardsInitiator)
}

// Initialize is a paid mutator transaction binding the contract method 0x485cc955.
//
// Solidity: function initialize(address initialOwner, address rewardsInitiator) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) Initialize(initialOwner common.Address, rewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.Initialize(&_ContractIncredibleDotProductServiceManager.TransactOpts, initialOwner, rewardsInitiator)
}

// RegisterOperatorToAVS is a paid mutator transaction binding the contract method 0x9926ee7d.
//
// Solidity: function registerOperatorToAVS(address operator, (bytes,bytes32,uint256) operatorSignature) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) RegisterOperatorToAVS(opts *bind.TransactOpts, operator common.Address, operatorSignature ISignatureUtilsMixinTypesSignatureWithSaltAndExpiry) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "registerOperatorToAVS", operator, operatorSignature)
}

// RegisterOperatorToAVS is a paid mutator transaction binding the contract method 0x9926ee7d.
//
// Solidity: function registerOperatorToAVS(address operator, (bytes,bytes32,uint256) operatorSignature) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) RegisterOperatorToAVS(operator common.Address, operatorSignature ISignatureUtilsMixinTypesSignatureWithSaltAndExpiry) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RegisterOperatorToAVS(&_ContractIncredibleDotProductServiceManager.TransactOpts, operator, operatorSignature)
}

// RegisterOperatorToAVS is a paid mutator transaction binding the contract method 0x9926ee7d.
//
// Solidity: function registerOperatorToAVS(address operator, (bytes,bytes32,uint256) operatorSignature) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) RegisterOperatorToAVS(operator common.Address, operatorSignature ISignatureUtilsMixinTypesSignatureWithSaltAndExpiry) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RegisterOperatorToAVS(&_ContractIncredibleDotProductServiceManager.TransactOpts, operator, operatorSignature)
}

// RemoveAdmin is a paid mutator transaction binding the contract method 0x1785f53c.
//
// Solidity: function removeAdmin(address admin) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) RemoveAdmin(opts *bind.TransactOpts, admin common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "removeAdmin", admin)
}

// RemoveAdmin is a paid mutator transaction binding the contract method 0x1785f53c.
//
// Solidity: function removeAdmin(address admin) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) RemoveAdmin(admin common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RemoveAdmin(&_ContractIncredibleDotProductServiceManager.TransactOpts, admin)
}

// RemoveAdmin is a paid mutator transaction binding the contract method 0x1785f53c.
//
// Solidity: function removeAdmin(address admin) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) RemoveAdmin(admin common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RemoveAdmin(&_ContractIncredibleDotProductServiceManager.TransactOpts, admin)
}

// RemoveAppointee is a paid mutator transaction binding the contract method 0xba550880.
//
// Solidity: function removeAppointee(address appointee, address target, bytes4 selector) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) RemoveAppointee(opts *bind.TransactOpts, appointee common.Address, target common.Address, selector [4]byte) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "removeAppointee", appointee, target, selector)
}

// RemoveAppointee is a paid mutator transaction binding the contract method 0xba550880.
//
// Solidity: function removeAppointee(address appointee, address target, bytes4 selector) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) RemoveAppointee(appointee common.Address, target common.Address, selector [4]byte) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RemoveAppointee(&_ContractIncredibleDotProductServiceManager.TransactOpts, appointee, target, selector)
}

// RemoveAppointee is a paid mutator transaction binding the contract method 0xba550880.
//
// Solidity: function removeAppointee(address appointee, address target, bytes4 selector) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) RemoveAppointee(appointee common.Address, target common.Address, selector [4]byte) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RemoveAppointee(&_ContractIncredibleDotProductServiceManager.TransactOpts, appointee, target, selector)
}

// RemovePendingAdmin is a paid mutator transaction binding the contract method 0x9da16d8e.
//
// Solidity: function removePendingAdmin(address pendingAdmin) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) RemovePendingAdmin(opts *bind.TransactOpts, pendingAdmin common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "removePendingAdmin", pendingAdmin)
}

// RemovePendingAdmin is a paid mutator transaction binding the contract method 0x9da16d8e.
//
// Solidity: function removePendingAdmin(address pendingAdmin) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) RemovePendingAdmin(pendingAdmin common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RemovePendingAdmin(&_ContractIncredibleDotProductServiceManager.TransactOpts, pendingAdmin)
}

// RemovePendingAdmin is a paid mutator transaction binding the contract method 0x9da16d8e.
//
// Solidity: function removePendingAdmin(address pendingAdmin) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) RemovePendingAdmin(pendingAdmin common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RemovePendingAdmin(&_ContractIncredibleDotProductServiceManager.TransactOpts, pendingAdmin)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RenounceOwnership(&_ContractIncredibleDotProductServiceManager.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.RenounceOwnership(&_ContractIncredibleDotProductServiceManager.TransactOpts)
}

// SetAppointee is a paid mutator transaction binding the contract method 0x1fdb0cfd.
//
// Solidity: function setAppointee(address appointee, address target, bytes4 selector) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) SetAppointee(opts *bind.TransactOpts, appointee common.Address, target common.Address, selector [4]byte) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "setAppointee", appointee, target, selector)
}

// SetAppointee is a paid mutator transaction binding the contract method 0x1fdb0cfd.
//
// Solidity: function setAppointee(address appointee, address target, bytes4 selector) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) SetAppointee(appointee common.Address, target common.Address, selector [4]byte) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.SetAppointee(&_ContractIncredibleDotProductServiceManager.TransactOpts, appointee, target, selector)
}

// SetAppointee is a paid mutator transaction binding the contract method 0x1fdb0cfd.
//
// Solidity: function setAppointee(address appointee, address target, bytes4 selector) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) SetAppointee(appointee common.Address, target common.Address, selector [4]byte) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.SetAppointee(&_ContractIncredibleDotProductServiceManager.TransactOpts, appointee, target, selector)
}

// SetClaimerFor is a paid mutator transaction binding the contract method 0xa0169ddd.
//
// Solidity: function setClaimerFor(address claimer) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) SetClaimerFor(opts *bind.TransactOpts, claimer common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "setClaimerFor", claimer)
}

// SetClaimerFor is a paid mutator transaction binding the contract method 0xa0169ddd.
//
// Solidity: function setClaimerFor(address claimer) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) SetClaimerFor(claimer common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.SetClaimerFor(&_ContractIncredibleDotProductServiceManager.TransactOpts, claimer)
}

// SetClaimerFor is a paid mutator transaction binding the contract method 0xa0169ddd.
//
// Solidity: function setClaimerFor(address claimer) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) SetClaimerFor(claimer common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.SetClaimerFor(&_ContractIncredibleDotProductServiceManager.TransactOpts, claimer)
}

// SetRewardsInitiator is a paid mutator transaction binding the contract method 0x3bc28c8c.
//
// Solidity: function setRewardsInitiator(address newRewardsInitiator) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) SetRewardsInitiator(opts *bind.TransactOpts, newRewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "setRewardsInitiator", newRewardsInitiator)
}

// SetRewardsInitiator is a paid mutator transaction binding the contract method 0x3bc28c8c.
//
// Solidity: function setRewardsInitiator(address newRewardsInitiator) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) SetRewardsInitiator(newRewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.SetRewardsInitiator(&_ContractIncredibleDotProductServiceManager.TransactOpts, newRewardsInitiator)
}

// SetRewardsInitiator is a paid mutator transaction binding the contract method 0x3bc28c8c.
//
// Solidity: function setRewardsInitiator(address newRewardsInitiator) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) SetRewardsInitiator(newRewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.SetRewardsInitiator(&_ContractIncredibleDotProductServiceManager.TransactOpts, newRewardsInitiator)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.TransferOwnership(&_ContractIncredibleDotProductServiceManager.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.TransferOwnership(&_ContractIncredibleDotProductServiceManager.TransactOpts, newOwner)
}

// UpdateAVSMetadataURI is a paid mutator transaction binding the contract method 0xa98fb355.
//
// Solidity: function updateAVSMetadataURI(string _metadataURI) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactor) UpdateAVSMetadataURI(opts *bind.TransactOpts, _metadataURI string) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.contract.Transact(opts, "updateAVSMetadataURI", _metadataURI)
}

// UpdateAVSMetadataURI is a paid mutator transaction binding the contract method 0xa98fb355.
//
// Solidity: function updateAVSMetadataURI(string _metadataURI) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerSession) UpdateAVSMetadataURI(_metadataURI string) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.UpdateAVSMetadataURI(&_ContractIncredibleDotProductServiceManager.TransactOpts, _metadataURI)
}

// UpdateAVSMetadataURI is a paid mutator transaction binding the contract method 0xa98fb355.
//
// Solidity: function updateAVSMetadataURI(string _metadataURI) returns()
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerTransactorSession) UpdateAVSMetadataURI(_metadataURI string) (*types.Transaction, error) {
	return _ContractIncredibleDotProductServiceManager.Contract.UpdateAVSMetadataURI(&_ContractIncredibleDotProductServiceManager.TransactOpts, _metadataURI)
}

// ContractIncredibleDotProductServiceManagerInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the ContractIncredibleDotProductServiceManager contract.
type ContractIncredibleDotProductServiceManagerInitializedIterator struct {
	Event *ContractIncredibleDotProductServiceManagerInitialized // Event containing the contract specifics and raw log

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
func (it *ContractIncredibleDotProductServiceManagerInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductServiceManagerInitialized)
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
		it.Event = new(ContractIncredibleDotProductServiceManagerInitialized)
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
func (it *ContractIncredibleDotProductServiceManagerInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductServiceManagerInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductServiceManagerInitialized represents a Initialized event raised by the ContractIncredibleDotProductServiceManager contract.
type ContractIncredibleDotProductServiceManagerInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerFilterer) FilterInitialized(opts *bind.FilterOpts) (*ContractIncredibleDotProductServiceManagerInitializedIterator, error) {

	logs, sub, err := _ContractIncredibleDotProductServiceManager.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductServiceManagerInitializedIterator{contract: _ContractIncredibleDotProductServiceManager.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductServiceManagerInitialized) (event.Subscription, error) {

	logs, sub, err := _ContractIncredibleDotProductServiceManager.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductServiceManagerInitialized)
				if err := _ContractIncredibleDotProductServiceManager.contract.UnpackLog(event, "Initialized", log); err != nil {
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
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerFilterer) ParseInitialized(log types.Log) (*ContractIncredibleDotProductServiceManagerInitialized, error) {
	event := new(ContractIncredibleDotProductServiceManagerInitialized)
	if err := _ContractIncredibleDotProductServiceManager.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductServiceManagerOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the ContractIncredibleDotProductServiceManager contract.
type ContractIncredibleDotProductServiceManagerOwnershipTransferredIterator struct {
	Event *ContractIncredibleDotProductServiceManagerOwnershipTransferred // Event containing the contract specifics and raw log

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
func (it *ContractIncredibleDotProductServiceManagerOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductServiceManagerOwnershipTransferred)
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
		it.Event = new(ContractIncredibleDotProductServiceManagerOwnershipTransferred)
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
func (it *ContractIncredibleDotProductServiceManagerOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductServiceManagerOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductServiceManagerOwnershipTransferred represents a OwnershipTransferred event raised by the ContractIncredibleDotProductServiceManager contract.
type ContractIncredibleDotProductServiceManagerOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*ContractIncredibleDotProductServiceManagerOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractIncredibleDotProductServiceManager.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductServiceManagerOwnershipTransferredIterator{contract: _ContractIncredibleDotProductServiceManager.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductServiceManagerOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractIncredibleDotProductServiceManager.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductServiceManagerOwnershipTransferred)
				if err := _ContractIncredibleDotProductServiceManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
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
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerFilterer) ParseOwnershipTransferred(log types.Log) (*ContractIncredibleDotProductServiceManagerOwnershipTransferred, error) {
	event := new(ContractIncredibleDotProductServiceManagerOwnershipTransferred)
	if err := _ContractIncredibleDotProductServiceManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdatedIterator is returned from FilterRewardsInitiatorUpdated and is used to iterate over the raw logs and unpacked data for RewardsInitiatorUpdated events raised by the ContractIncredibleDotProductServiceManager contract.
type ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdatedIterator struct {
	Event *ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdated // Event containing the contract specifics and raw log

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
func (it *ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdated)
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
		it.Event = new(ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdated)
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
func (it *ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdated represents a RewardsInitiatorUpdated event raised by the ContractIncredibleDotProductServiceManager contract.
type ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdated struct {
	PrevRewardsInitiator common.Address
	NewRewardsInitiator  common.Address
	Raw                  types.Log // Blockchain specific contextual infos
}

// FilterRewardsInitiatorUpdated is a free log retrieval operation binding the contract event 0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3.
//
// Solidity: event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerFilterer) FilterRewardsInitiatorUpdated(opts *bind.FilterOpts) (*ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdatedIterator, error) {

	logs, sub, err := _ContractIncredibleDotProductServiceManager.contract.FilterLogs(opts, "RewardsInitiatorUpdated")
	if err != nil {
		return nil, err
	}
	return &ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdatedIterator{contract: _ContractIncredibleDotProductServiceManager.contract, event: "RewardsInitiatorUpdated", logs: logs, sub: sub}, nil
}

// WatchRewardsInitiatorUpdated is a free log subscription operation binding the contract event 0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3.
//
// Solidity: event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerFilterer) WatchRewardsInitiatorUpdated(opts *bind.WatchOpts, sink chan<- *ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdated) (event.Subscription, error) {

	logs, sub, err := _ContractIncredibleDotProductServiceManager.contract.WatchLogs(opts, "RewardsInitiatorUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdated)
				if err := _ContractIncredibleDotProductServiceManager.contract.UnpackLog(event, "RewardsInitiatorUpdated", log); err != nil {
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

// ParseRewardsInitiatorUpdated is a log parse operation binding the contract event 0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3.
//
// Solidity: event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator)
func (_ContractIncredibleDotProductServiceManager *ContractIncredibleDotProductServiceManagerFilterer) ParseRewardsInitiatorUpdated(log types.Log) (*ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdated, error) {
	event := new(ContractIncredibleDotProductServiceManagerRewardsInitiatorUpdated)
	if err := _ContractIncredibleDotProductServiceManager.contract.UnpackLog(event, "RewardsInitiatorUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
