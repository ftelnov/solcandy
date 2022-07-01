use crate::error::SolcandyError;
use crate::programs::candy_v2_program;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

/// Version of candy machine, since metaplex introduced new one.
#[derive(Copy, Clone, Debug)]
pub enum CandyVersion {
    V1,
    V2,
}

impl FromStr for CandyVersion {
    type Err = SolcandyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "v1" => Ok(CandyVersion::V1),
            "2" | "v2" => Ok(CandyVersion::V2),
            _ => Err(SolcandyError::InvalidCandyVersion),
        }
    }
}

pub struct CandyMachine {
    key: Pubkey,
    version: CandyVersion,
}

impl CandyMachine {
    pub fn new(key: &Pubkey, version: CandyVersion) -> Self {
        Self { key: *key, version }
    }

    pub fn key(&self) -> &Pubkey {
        &self.key
    }

    pub fn version(&self) -> CandyVersion {
        self.version
    }

    pub fn find_creator(&self) -> Pubkey {
        match self.version {
            CandyVersion::V1 => self.key,
            CandyVersion::V2 => {
                Pubkey::find_program_address(
                    &[b"candy_machine", &self.key().to_bytes()],
                    &candy_v2_program(),
                )
                .0
            }
        }
    }
}
