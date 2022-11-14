use chrono;
use solana_client::client_error::ClientError;
use solana_sdk::{pubkey::ParsePubkeyError, signature::ParseSignatureError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssignmentError {
    #[error("Unable to parse the date from String")]
    StringToDateParseError(#[from] chrono::format::ParseError),

    #[error("Unable to get data")]
    RpcClientError(#[from] ClientError),

    #[error("Unable to parse pubkey from string")]
    PubkeyFromStrError(#[from] ParsePubkeyError),

    #[error("Unable to parse signature from string")]
    SignatureFromStrError(#[from] ParseSignatureError),

    #[error("Block time missing from transaction")]
    BlockTimeMissing,

    #[error("Unable to get data out of meta")]
    EmptyMetaError,

    #[error("Empty token balance")]
    EmptyTokenBalanceError,

    #[error("Unable to get UiTransaction match arm")]
    UnableToGetUiTransactionError,

    #[error("Unable to get token balances")]
    UnableToGetTokenBalancesError,

    #[error("Unable to get token account")]
    UnableToGetTokenAccountOwnerError,
}
