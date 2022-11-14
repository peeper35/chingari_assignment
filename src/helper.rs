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

// get_client() function returns the RpcClient object
// for making rpc requests
pub fn get_client() -> RpcClient {
    // return the RpcClient with confirmed commitment
    RpcClient::new_with_commitment(
        String::from("https://api.mainnet-beta.solana.com"),
        CommitmentConfig::confirmed(),
    )
}

#[derive(Debug, Clone)]
// new user data struct, for wrapping new user's information at a place
pub struct NewUserData {
    pub signatures: Vec<String>,
    pub owner: String,
    pub pre_token_balance: f64,
    pub post_token_balance: f64,
}

// this function is useful to filter the signatures date
// according to the start date and end date
// only signature date which are equal to or greater than start date
// and equal to or lower than end date will be returned
pub fn filter_data_start_end_date(
    txs: Vec<RpcConfirmedTransactionStatusWithSignature>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<RpcConfirmedTransactionStatusWithSignature>, AssignmentError> {
    let filtered_data: Vec<RpcConfirmedTransactionStatusWithSignature> = txs
        .into_iter()
        .filter(|txn| {
            // get the blocktime from the signature
            let txn_date = Local
                // convert the utc timestamp to local date
                .timestamp_opt(
                    txn.block_time
                        .ok_or(AssignmentError::BlockTimeMissing)
                        .unwrap(),
                    0,
                )
                .unwrap()
                .date_naive();

            // filter the date according to start date and end date
            // traits PartialOrd, Ord, PartialEq, Eq
            // are implemented on NaiveDate hence we can compare directly
            txn_date >= start_date && txn_date <= end_date
        })
        .collect();

    Ok(filtered_data)
}

// helper function used to unwrap
// user's pre token balance and post token balance
// from the OptionSerializer type
pub fn get_data_out_of_option_serializer(
    status_meta: &EncodedConfirmedTransactionWithStatusMeta,
) -> Result<
    (
        Vec<UiTransactionTokenBalance>,
        Vec<UiTransactionTokenBalance>,
    ),
    AssignmentError,
> {
    // get the meta data from the transaction
    let meta_data = status_meta
        .transaction
        .meta
        .as_ref()
        .ok_or(AssignmentError::EmptyMetaError)?;

    // fetch the pre token balance from meta data
    let pre_token_balance = match &meta_data.pre_token_balances {
        OptionSerializer::Some(val) => val,
        _ => return Err(AssignmentError::EmptyTokenBalanceError),
    };

    // fetch the post token balance from meta data
    let post_token_balance = match &meta_data.post_token_balances {
        OptionSerializer::Some(val) => val,
        _ => return Err(AssignmentError::EmptyTokenBalanceError),
    };

    Ok((pre_token_balance.to_owned(), post_token_balance.to_owned()))
}

// check if the user is new
// if the user has recently signed up on chingari's android app
// a Keypair (Wallet) will be created for the user
// along with that ATA for gari token will be created
// and 0.000000001 gari token will be airdropped into the ATA
pub fn check_new_user(
    data: &EncodedConfirmedTransactionWithStatusMeta,
) -> Result<Option<NewUserData>, AssignmentError> {
    let (pre_token_balances, post_token_balances) = get_data_out_of_option_serializer(data)?;

    // check if there is no pre_token_balance
    // and post_token_balance is 0.000000001
    if pre_token_balances.is_empty() && post_token_balances.len() == 1 {
        // get the ATA owner
        let post_bal_owner = match &post_token_balances[0].owner {
            OptionSerializer::Some(val) => val,
            _ => return Err(AssignmentError::UnableToGetTokenAccountOwnerError),
        };

        // get the signature(s) for the transaction
        let ui_transaction = match &data.transaction.transaction {
            solana_transaction_status::EncodedTransaction::Json(ui_transaction) => ui_transaction,
            _ => return Err(AssignmentError::UnableToGetUiTransactionError),
        };

        // return the new user data wrapped in NewUserData struct
        return Ok(Some(NewUserData {
            signatures: ui_transaction.signatures.to_owned(),
            owner: post_bal_owner.to_owned(),
            pre_token_balance: 0.000000000,
            post_token_balance: 0.000000001,
        }));
    }

    Ok(None)
}

// print the new user's data to the std out
pub fn print_data(user_data: &NewUserData) -> Result<(), AssignmentError> {
    println!("New User -");
    println!("\tSignature(s) - {:?}", user_data.signatures);
    println!("\tOwner - {}", user_data.owner);
    println!("\tPre Token Balance - {}", user_data.pre_token_balance);
    println!("\tPost Token Balance - {}", user_data.post_token_balance);
    println!("-----------");

    Ok(())
}
