use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolcandyError {
    #[error("Invalid candy version, unable to parse it properly.")]
    InvalidCandyVersion,

    #[error("Failed to fetch accounts from solana network.")]
    FetchAccountsError,
}

pub type SolcandyResult<T> = Result<T, SolcandyError>;
