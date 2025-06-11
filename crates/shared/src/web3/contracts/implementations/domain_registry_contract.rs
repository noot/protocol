use crate::web3::contracts::constants::addresses::DOMAIN_REGISTRY_ADDRESS;
use crate::web3::contracts::core::contract::Contract;
use crate::web3::contracts::core::error::{ContractResult, Error};
use alloy::dyn_abi::DynSolValue;
use alloy::primitives::{Address, U256};

pub struct Domain {
    pub domain_id: U256,
    pub name: String,
    pub validation_logic: Address,
    pub domain_parameters_uri: String,
}

#[derive(Clone)]
pub struct DomainRegistryContract<P: alloy_provider::Provider> {
    instance: Contract<P>,
}

impl<P: alloy_provider::Provider> DomainRegistryContract<P> {
    pub fn new(provider: P, abi_file_path: &str) -> Self {
        let instance = Contract::new(DOMAIN_REGISTRY_ADDRESS, provider, abi_file_path);
        Self { instance }
    }

    pub async fn get_domain(&self, domain_id: u32) -> ContractResult<Domain> {
        let result = self
            .instance
            .instance()
            .function("get", &[U256::from(domain_id).into()])?
            .call()
            .await?;

        let domain_info_tuple: &[DynSolValue] = result
            .first()
            .ok_or_else(|| Error::InvalidResponse("Failed to get domain info tuple".to_string()))?
            .as_tuple()
            .ok_or_else(|| Error::InvalidResponse("Failed to convert to tuple".to_string()))?;

        let domain_id: U256 = domain_info_tuple[0]
            .as_uint()
            .ok_or_else(|| Error::DecodingError("Failed to get domain ID".to_string()))?
            .0;
        let name: String = domain_info_tuple[1]
            .as_str()
            .ok_or_else(|| Error::DecodingError("Failed to get domain name".to_string()))?
            .to_string();
        let validation_logic: Address = domain_info_tuple[2].as_address().ok_or_else(|| {
            Error::DecodingError("Failed to get validation logic address".to_string())
        })?;
        let domain_parameters_uri: String = domain_info_tuple[3]
            .as_str()
            .ok_or_else(|| Error::DecodingError("Failed to get domain parameters URI".to_string()))?
            .to_string();

        Ok(Domain {
            domain_id,
            name,
            validation_logic,
            domain_parameters_uri,
        })
    }
}
