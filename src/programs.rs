use solana_program::pubkey::Pubkey;
use std::str::FromStr;

/// Metadata program key, got it from their official git repo.
const METADATA_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

/// Key of the new candy program.
const CANDY_PROGRAM_V2: &str = "cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ";

// Since we surely know that these are real keys, we simply unwrap the results.
pub fn metadata_program() -> Pubkey {
    Pubkey::from_str(METADATA_PROGRAM_ID).unwrap()
}

pub fn candy_v2_program() -> Pubkey {
    Pubkey::from_str(CANDY_PROGRAM_V2).unwrap()
}
