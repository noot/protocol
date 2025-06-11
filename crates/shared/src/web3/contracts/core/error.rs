use alloy::primitives::Address;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    // Initialization errors
    AbiParseError(String),
    ArtifactReadError(String),

    // Contract interaction errors
    CallError(String),
    TransactionError(String),

    // Data parsing errors
    DecodingError(String),
    InvalidResponse(String),

    // Business logic errors
    ProviderNotFound(Address),
    NodeNotRegistered { provider: Address, node: Address },
    InvalidProviderState(String),

    // Generic errors
    Web3Error(String),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Initialization errors
            Error::AbiParseError(msg) => write!(f, "Failed to parse ABI: {msg}"),
            Error::ArtifactReadError(msg) => write!(f, "Failed to read artifact: {msg}"),

            // Contract interaction errors
            Error::CallError(msg) => write!(f, "Contract call failed: {msg}"),
            Error::TransactionError(msg) => write!(f, "Transaction failed: {msg}"),

            // Data parsing errors
            Error::DecodingError(msg) => write!(f, "Failed to decode data: {msg}"),
            Error::InvalidResponse(msg) => write!(f, "Invalid contract response: {msg}"),

            // Business logic errors
            Error::ProviderNotFound(address) => {
                write!(f, "Provider not found: {address:?}")
            }
            Error::NodeNotRegistered { provider, node } => {
                write!(f, "Node {node:?} not registered for provider {provider:?}")
            }
            Error::InvalidProviderState(msg) => {
                write!(f, "Invalid provider state: {msg}")
            }

            // Generic errors
            Error::Web3Error(msg) => write!(f, "Web3 error: {msg}"),
            Error::Other(e) => write!(f, "Other error: {e}"),
        }
    }
}

// Convenient type alias for Result with Error
pub type ContractResult<T> = Result<T, Error>;

// Conversion implementations for common error types
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::ArtifactReadError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::AbiParseError(err.to_string())
    }
}

impl From<alloy::contract::Error> for Error {
    fn from(err: alloy::contract::Error) -> Self {
        Error::CallError(err.to_string())
    }
}
