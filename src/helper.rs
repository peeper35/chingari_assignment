use chrono::{Local, NaiveDate, TimeZone};
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedConfirmedTransactionWithStatusMeta,
    UiTransactionTokenBalance,
};

use crate::error::AssignmentError;

pub fn get_client() -> RpcClient {
    RpcClient::new_with_commitment(
        String::from("https://api.mainnet-beta.solana.com"),
        CommitmentConfig::confirmed(),
    )
}

#[derive(Debug, Clone)]
pub struct NewUserData {
    pub signatures: Vec<String>,
    pub owner: String,
    pub pre_token_balance: f64,
    pub post_token_balance: f64,
}

pub fn filter_data_start_end_date(
    txs: Vec<RpcConfirmedTransactionStatusWithSignature>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<RpcConfirmedTransactionStatusWithSignature>, AssignmentError> {
    let filtered_data: Vec<RpcConfirmedTransactionStatusWithSignature> = txs
        .into_iter()
        .filter(|txn| {
            let txn_date = Local
                .timestamp_opt(
                    txn.block_time
                        .ok_or(AssignmentError::BlockTimeMissing)
                        .unwrap(),
                    0,
                )
                .unwrap()
                .date_naive();

            txn_date >= start_date && txn_date <= end_date
        })
        .collect();

    Ok(filtered_data)
}

pub fn get_data_out_of_option_serializer(
    status_meta: &EncodedConfirmedTransactionWithStatusMeta,
) -> Result<
    (
        Vec<UiTransactionTokenBalance>,
        Vec<UiTransactionTokenBalance>,
    ),
    AssignmentError,
> {
    let meta_data = status_meta
        .transaction
        .meta
        .as_ref()
        .ok_or(AssignmentError::EmptyMetaError)?;
    let pre_token_balance = match &meta_data.pre_token_balances {
        OptionSerializer::Some(val) => val,
        _ => return Err(AssignmentError::EmptyTokenBalanceError),
    };

    let post_token_balance = match &meta_data.post_token_balances {
        OptionSerializer::Some(val) => val,
        _ => return Err(AssignmentError::EmptyTokenBalanceError),
    };

    Ok((pre_token_balance.to_owned(), post_token_balance.to_owned()))
}

pub fn check_new_user(
    data: &EncodedConfirmedTransactionWithStatusMeta,
) -> Result<Option<NewUserData>, AssignmentError> {
    let (pre_token_balances, post_token_balances) = get_data_out_of_option_serializer(&data)?;

    if pre_token_balances.len() == 0 && post_token_balances.len() == 1 {
        let post_bal_owner = match &post_token_balances[0].owner {
            OptionSerializer::Some(val) => val,
            _ => return Err(AssignmentError::UnableToGetTokenAccountOwnerError),
        };

        let ui_transaction = match &data.transaction.transaction {
            solana_transaction_status::EncodedTransaction::Json(ui_transaction) => ui_transaction,
            _ => return Err(AssignmentError::UnableToGetUiTransactionError),
        };

        return Ok(Some(NewUserData {
            signatures: ui_transaction.signatures.to_owned(),
            owner: post_bal_owner.to_owned(),
            pre_token_balance: 0.000000000,
            post_token_balance: 0.000000001,
        }));
    }

    Ok(None)
}

pub fn print_data(user_data: &NewUserData) -> Result<(), AssignmentError> {
    println!("New User -");
    println!("\tSignature(s) - {:?}", user_data.signatures);
    println!("\tOwner - {}", user_data.owner);
    println!("\tPre Token Balance - {}", user_data.pre_token_balance);
    println!("\tPost Token Balance - {}", user_data.post_token_balance);
    println!("-----------");

    Ok(())
}
