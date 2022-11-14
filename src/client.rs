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

// added get_signature() method to get the transaction signatures related to the gari token mint
// this function can fetch upto 1000 signatures
pub async fn get_signatures(
    client: &RpcClient,
) -> Result<Vec<RpcConfirmedTransactionStatusWithSignature>, AssignmentError> {
    // created the config object
    // to fetch 1000 signatures for gari token
    let config = GetConfirmedSignaturesForAddress2Config {
        before: None,
        until: None,
        limit: Some(1000),
        commitment: Some(CommitmentConfig::confirmed()),
    };

    // return fetched signatures
    Ok(client
        .get_signatures_for_address_with_config(&Pubkey::from_str(PUBKEY)?, config)
        .await?)
}

// added function to get the transaction from the signature
// this function also retrives the new users by parsing the transaction data
// prints the new user's data
pub async fn get_transactions_check_print_new_user(
    client: &RpcClient,
    signatures: &Vec<RpcConfirmedTransactionStatusWithSignature>,
) -> Result<(), AssignmentError> {
    for sig in signatures {
        // get the transaction from the data
        let tx = client
            .get_transaction(
                &Signature::from_str(&sig.signature)?,
                UiTransactionEncoding::JsonParsed,
            )
            .await?;

        // print the new user's data
        if let Some(user_data) = helper::check_new_user(&tx)? {
            helper::print_data(&user_data)?;
        }
    }

    Ok(())
}
