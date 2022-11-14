use std::str::FromStr;

use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
    rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;

use crate::error::AssignmentError;
use crate::helper;

const PUBKEY: &str = "CKaKtYvz6dKPyMvYq9Rh3UBrnNqYZAyd7iF4hJtjUvks";

pub async fn get_signatures(
    client: &RpcClient,
) -> Result<Vec<RpcConfirmedTransactionStatusWithSignature>, AssignmentError> {
    let config = GetConfirmedSignaturesForAddress2Config {
        before: None,
        until: None,
        limit: Some(1000),
        commitment: Some(CommitmentConfig::confirmed()),
    };

    Ok(client
        .get_signatures_for_address_with_config(&Pubkey::from_str(PUBKEY)?, config)
        .await?)
}

pub async fn get_transactions_check_print_new_user(
    client: &RpcClient,
    signatures: &Vec<RpcConfirmedTransactionStatusWithSignature>,
) -> Result<(), AssignmentError> {
    for sig in signatures {
        let tx = client
            .get_transaction(
                &Signature::from_str(&sig.signature)?,
                UiTransactionEncoding::JsonParsed,
            )
            .await?;

        if let Some(user_data) = helper::check_new_user(&tx)? {
            helper::print_data(&user_data)?;
        }
    }

    Ok(())
}
