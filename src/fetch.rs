use crate::error::{SolcandyError, SolcandyResult};
use crate::programs::metadata_program;
use crate::state::CandyMachine;
use async_trait::async_trait;
use solana_account_decoder::{UiAccountEncoding, UiDataSliceConfig};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_program::pubkey::Pubkey;
use solana_sdk::account::ReadableAccount;

async fn fetch_mint_keys(
    client: &RpcClient,
    first_creator: &Pubkey,
) -> SolcandyResult<Vec<Pubkey>> {
    let creator_bytes = first_creator.to_bytes().to_vec();
    let fetched = client
        .get_program_accounts_with_config(
            &metadata_program(),
            RpcProgramAccountsConfig {
                filters: Some(vec![
                    RpcFilterType::DataSize(679),
                    RpcFilterType::Memcmp(Memcmp {
                        offset: 326,
                        bytes: MemcmpEncodedBytes::Bytes(creator_bytes),
                        encoding: None,
                    }),
                ]),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    data_slice: Some(UiDataSliceConfig {
                        offset: 33,
                        length: 32,
                    }),
                    commitment: None,
                },
                with_context: None,
            },
        )
        .await
        .map_err(|_| SolcandyError::FetchAccountsError)?;
    Ok(fetched
        .into_iter()
        .map(|(_, key)| Pubkey::new(key.data()))
        .collect())
}

async fn fetch_candy_mints(
    client: &RpcClient,
    candy: &CandyMachine,
) -> SolcandyResult<Vec<Pubkey>> {
    let creator_key = candy.find_creator();
    fetch_mint_keys(client, &creator_key).await
}

#[async_trait]
pub trait CandyFetch {
    /// Fetches keys of all mints that were distributed by candy machine.
    async fn list_keys(&self, client: &RpcClient) -> SolcandyResult<Vec<Pubkey>>;
}

#[async_trait]
impl CandyFetch for CandyMachine {
    async fn list_keys(&self, client: &RpcClient) -> SolcandyResult<Vec<Pubkey>> {
        fetch_candy_mints(client, self).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::CandyVersion;
    use std::str::FromStr;

    struct TestContext {
        candy_machine: CandyMachine,
        mints_amount: usize,
        client: RpcClient,
    }

    fn get_devnet_client() -> RpcClient {
        RpcClient::new("https://api.devnet.solana.com".to_string())
    }

    fn unpack_key(raw: &str) -> Pubkey {
        Pubkey::from_str(raw).unwrap()
    }

    impl TestContext {
        fn new_devnet(version: CandyVersion) -> Self {
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
                candy_machine: CandyMachine::new(&key, version),
                mints_amount: amount,
                client: get_devnet_client(),
            }
        }
    }

    #[tokio::test]
    async fn test_v1_devnet() {
        let ctx = TestContext::new_devnet(CandyVersion::V1);
        let tokens = ctx.candy_machine.list_keys(&ctx.client).await.unwrap();
        assert_eq!(tokens.len(), ctx.mints_amount)
    }

    #[tokio::test]
    async fn test_v2_devnet() {
        let ctx = TestContext::new_devnet(CandyVersion::V2);
        let tokens = ctx.candy_machine.list_keys(&ctx.client).await.unwrap();
        assert_eq!(tokens.len(), ctx.mints_amount)
    }
}
