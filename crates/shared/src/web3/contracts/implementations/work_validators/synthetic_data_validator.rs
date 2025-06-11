use crate::web3::contracts::core::contract::Contract;
use crate::web3::contracts::core::error::{ContractResult, Error};
use alloy::{
    dyn_abi::{DynSolValue, Word},
    primitives::{Address, U256},
};
use log::debug;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone)]
pub struct SyntheticDataWorkValidator<P: alloy_provider::Provider> {
    pub instance: Contract<P>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct WorkInfo {
    pub provider: Address,
    pub node_id: Address,
    pub timestamp: u64,
    pub work_units: U256,
}

impl<P: alloy_provider::Provider> SyntheticDataWorkValidator<P> {
    pub fn new(address: Address, provider: P, abi_file_path: &str) -> Self {
        let instance = Contract::new(address, provider, abi_file_path);
        Self { instance }
    }

    pub async fn get_work_keys(&self, pool_id: U256) -> ContractResult<Vec<String>> {
        let result = self
            .instance
            .instance()
            .function("getWorkKeys", &[pool_id.into()])?
            .call()
            .await?;

        let array_value = result.into_iter().next().ok_or_else(|| {
            Error::InvalidResponse("No result returned from getWorkKeys".to_string())
        })?;

        let array = array_value
            .as_array()
            .ok_or_else(|| Error::InvalidResponse("Result is not an array".to_string()))?;

        // Map each value to a hex string
        let work_keys = array
            .iter()
            .map(|value| {
                let bytes = value
                    .as_fixed_bytes()
                    .ok_or_else(|| Error::DecodingError("Value is not fixed bytes".to_string()))?;

                // Ensure we have exactly 32 bytes
                if bytes.0.len() != 32 {
                    return Err(Error::DecodingError(format!(
                        "Expected 32 bytes, got {}",
                        bytes.0.len()
                    )));
                }

                // Convert bytes to string
                Ok(hex::encode(bytes.0))
            })
            .collect::<ContractResult<Vec<String>>>()?;

        Ok(work_keys)
    }

    pub async fn get_work_info(&self, pool_id: U256, work_key: &str) -> ContractResult<WorkInfo> {
        // Convert work_key from hex string to bytes32
        debug!("Processing work key: {}", work_key);
        let work_key_bytes = hex::decode(work_key)
            .map_err(|e| Error::DecodingError(format!("Failed to decode hex work key: {}", e)))?;
        if work_key_bytes.len() != 32 {
            return Err(Error::DecodingError(
                "Work key must be 32 bytes".to_string(),
            ));
        }
        debug!("Decoded work key bytes: {:?}", work_key_bytes);

        let fixed_bytes = DynSolValue::FixedBytes(Word::from_slice(&work_key_bytes), 32);

        let result = self
            .instance
            .instance()
            .function("getWorkInfo", &[pool_id.into(), fixed_bytes])?
            .call()
            .await?;
        debug!("Got work info result: {:?}", result);

        let tuple = result.into_iter().next().ok_or_else(|| {
            Error::InvalidResponse("No result returned from getWorkInfo".to_string())
        })?;

        let tuple_array = tuple
            .as_tuple()
            .ok_or_else(|| Error::InvalidResponse("Result is not a tuple".to_string()))?;
        if tuple_array.len() != 4 {
            return Err(Error::InvalidResponse("Invalid tuple length".to_string()));
        }

        let provider = tuple_array[0]
            .as_address()
            .ok_or_else(|| Error::DecodingError("Provider is not an address".to_string()))?;

        let node_id = tuple_array[1]
            .as_address()
            .ok_or_else(|| Error::DecodingError("Node ID is not an address".to_string()))?;

        let timestamp = u64::try_from(
            tuple_array[2]
                .as_uint()
                .ok_or_else(|| Error::DecodingError("Timestamp is not a uint".to_string()))?
                .0,
        )
        .map_err(|_| Error::DecodingError("Timestamp conversion failed".to_string()))?;

        let work_units = tuple_array[3]
            .as_uint()
            .ok_or_else(|| Error::DecodingError("Work units is not a uint".to_string()))?
            .0;

        Ok(WorkInfo {
            provider,
            node_id,
            timestamp,
            work_units,
        })
    }

    pub async fn get_work_since(
        &self,
        pool_id: U256,
        timestamp: U256,
    ) -> ContractResult<Vec<String>> {
        let result = self
            .instance
            .instance()
            .function("getWorkSince", &[pool_id.into(), timestamp.into()])?
            .call()
            .await?;

        let array_value = result.into_iter().next().ok_or_else(|| {
            Error::InvalidResponse("No result returned from getWorkSince".to_string())
        })?;

        let array = array_value
            .as_array()
            .ok_or_else(|| Error::InvalidResponse("Result is not an array".to_string()))?;

        let work_keys = array
            .iter()
            .map(|value| {
                let bytes = value
                    .as_fixed_bytes()
                    .ok_or_else(|| Error::DecodingError("Value is not fixed bytes".to_string()))?;

                // Ensure we have exactly 32 bytes
                if bytes.0.len() != 32 {
                    return Err(Error::DecodingError(format!(
                        "Expected 32 bytes, got {}",
                        bytes.0.len()
                    )));
                }

                // Convert bytes to string
                Ok(hex::encode(bytes.0))
            })
            .collect::<ContractResult<Vec<String>>>()?;

        Ok(work_keys)
    }
}
