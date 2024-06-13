use alloy_primitives::{Address, FixedBytes, U256};
use eigen_types::operator::Operator;
use eigen_utils::{
    binding::{AVSDirectory, DelegationManager, ISlasher, IStrategy, IERC20},
    get_provider,
};

#[derive(Debug, Clone)]
pub struct ELChainReader {
    slasher: Address,
    delegation_manager: Address,
    avs_directory: Address,
    provider: String,
}

impl ELChainReader {
    pub fn new(
        slasher: Address,
        delegation_manager: Address,
        avs_directory: Address,
        provider: String,
    ) -> Self {
        ELChainReader {
            slasher,
            delegation_manager,
            avs_directory,
            provider,
        }
    }

    pub async fn build(
        delegation_manager: Address,
        avs_directory: Address,
        client: &String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = get_provider(client);

        let contract_delegation_manager = DelegationManager::new(delegation_manager, provider);

        let slasher = contract_delegation_manager.slasher().call().await?;

        let DelegationManager::slasherReturn { _0: slasher_addr } = slasher;

        let strategy_manager_addr = contract_delegation_manager.strategyManager().call().await?;

        let DelegationManager::strategyManagerReturn {
            _0: strategy_manager,
        } = strategy_manager_addr;
        Ok(Self {
            avs_directory,
            slasher: slasher_addr,
            delegation_manager,
            provider: client.to_string(),
        })
    }

    pub async fn calculate_delegation_approval_digest_hash(
        &self,
        staker: Address,
        operator: Address,
        delegation_approver: Address,
        approve_salt: FixedBytes<32>,
        expiry: U256,
    ) -> Result<FixedBytes<32>, Box<dyn std::error::Error>> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let delegation_approval_digest_hash = contract_delegation_manager
            .calculateDelegationApprovalDigestHash(
                staker,
                operator,
                delegation_approver,
                approve_salt,
                expiry,
            )
            .call()
            .await?;

        let DelegationManager::calculateDelegationApprovalDigestHashReturn { _0: digest_hash } =
            delegation_approval_digest_hash;

        Ok(digest_hash)
    }

    pub async fn calculate_operator_avs_registration_digest_hash(
        &self,
        operator: Address,
        avs: Address,
        salt: FixedBytes<32>,
        expiry: U256,
    ) -> Result<FixedBytes<32>, Box<dyn std::error::Error>> {
        let provider = get_provider(&self.provider);

        let contract_avs_directory = AVSDirectory::new(self.avs_directory, provider);

        let operator_avs_registration_digest_hash = contract_avs_directory
            .calculateOperatorAVSRegistrationDigestHash(operator, avs, salt, expiry)
            .call()
            .await?;

        let AVSDirectory::calculateOperatorAVSRegistrationDigestHashReturn { _0: avs_hash } =
            operator_avs_registration_digest_hash;

        Ok(avs_hash)
    }

    pub async fn get_operator_shares_in_strategy(
        &self,
        operator_addr: Address,
        strategy_addr: Address,
    ) -> Result<U256, Box<dyn std::error::Error>> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let operator_shares_in_strategy = contract_delegation_manager
            .operatorShares(operator_addr, strategy_addr)
            .call()
            .await?;
        let DelegationManager::operatorSharesReturn { _0: shares } = operator_shares_in_strategy;

        Ok(shares)
    }

    pub async fn operator_is_frozen(
        &self,
        operator_addr: Address,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let provider = get_provider(&self.provider);

        let contract_slasher = ISlasher::new(self.slasher, provider);

        let operator_is_frozen = contract_slasher.isFrozen(operator_addr).call().await?;

        let ISlasher::isFrozenReturn { _0: is_froze } = operator_is_frozen;
        Ok(is_froze)
    }

    pub async fn service_manager_can_slash_operator_until_block(
        &self,
        operator_addr: Address,
        service_manager_addr: Address,
    ) -> Result<u32, Box<dyn std::error::Error>> {
        let provider = get_provider(&self.provider);

        let contract_slasher = ISlasher::new(self.slasher, provider);

        let service_manager_can_slash_operator_until_block = contract_slasher
            .contractCanSlashOperatorUntilBlock(operator_addr, service_manager_addr)
            .call()
            .await?;

        let ISlasher::contractCanSlashOperatorUntilBlockReturn { _0: can_slash } =
            service_manager_can_slash_operator_until_block;

        Ok(can_slash)
    }

    pub async fn get_strategy_and_underlying_erc20_token(
        &self,
        strategy_addr: Address,
    ) -> Result<(Address, Address, Address), Box<dyn std::error::Error>> {
        let provider = get_provider(&self.provider);

        let contract_strategy = IStrategy::new(strategy_addr, &provider);

        let underlying_token = contract_strategy.underlyingToken().call().await?;

        let IStrategy::underlyingTokenReturn {
            _0: underlying_token_addr,
        } = underlying_token;

        let contract_ierc20 = IERC20::new(underlying_token_addr, &provider);

        return Ok((
            strategy_addr,
            underlying_token_addr,
            *contract_ierc20.address(),
        ));
    }

    pub async fn get_operator_details(
        &self,
        operator: Address,
    ) -> Result<Operator, Box<dyn std::error::Error>> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager =
            DelegationManager::new(self.delegation_manager, &provider);

        let operator_det = contract_delegation_manager
            .operatorDetails(operator)
            .call()
            .await?;

        let DelegationManager::operatorDetailsReturn {
            _0: operator_details,
        } = operator_det;

        Ok(Operator::new(
            operator,
            operator_details.earningsReceiver,
            operator_details.delegationApprover,
            operator_details.stakerOptOutWindowBlocks,
            None,
        ))
    }

    pub async fn is_operator_registered(
        &self,
        operator: Address,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let is_operator = contract_delegation_manager
            .isOperator(operator)
            .call()
            .await?;

        let DelegationManager::isOperatorReturn { _0: is_operator_is } = is_operator;
        Ok(is_operator_is)
    }
}
