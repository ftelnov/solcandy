use crate::error::SolcandyError;
use crate::programs::candy_v2_program;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

/// Version of candy machine, since metaplex introduced new one.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CandyVersion {
    #[cfg_attr(feature = "serde", serde(alias = "v1"), serde(alias = "1"))]
    V1,

    #[cfg_attr(feature = "serde", serde(alias = "v2"), serde(alias = "2"))]
    V2,
}

impl FromStr for CandyVersion {
    type Err = SolcandyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "v1" | "V1" => Ok(CandyVersion::V1),
            "2" | "v2" | "V2" => Ok(CandyVersion::V2),
            _ => Err(SolcandyError::InvalidCandyVersion),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(any(feature = "dev", test))]
pub mod dev {
    use super::*;
    use solana_program::pubkey::Pubkey;

    fn unpack_key(raw: &str) -> Pubkey {
        Pubkey::from_str(raw).unwrap()
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct CandySample {
        /// Sample candy machine.
        pub candy: CandyMachine,

        /// Amount of tokens that are already presented on the candy machine.
        ///
        /// Suitable for assertion.
        pub token_amount: usize,
    }

    impl CandySample {
        /// Provides devnet samples for each versions.
        pub fn new_devnet(version: CandyVersion) -> Self {
            let (key, amount) = match version {
                CandyVersion::V1 => (
                    unpack_key("4xGR6jwwAhBebU9ugdq7pkzsz7Q1P3Djg72Fby1xmUkA"),
                    14,
                ),
                CandyVersion::V2 => (
                    unpack_key("C24whbLeUARPsuiJAkZ41dxrmRKBhqzQqQ6hfLMRY1mD"),
                    14,
                ),
            };
            Self {
                candy: CandyMachine::new(&key, version),
                token_amount: amount,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[cfg(feature = "serde")]
    #[test]
    fn it_deserializes_candy_version() {
        let value = json!("1");
        let version: CandyVersion = serde_json::from_value(value).unwrap();
        assert_eq!(version, CandyVersion::V1)
    }
}
