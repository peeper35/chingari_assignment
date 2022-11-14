pub mod args;
pub mod client;
pub mod error;
pub mod helper;

use args::ParsedData;
use clap::Parser;
use error::AssignmentError;

#[tokio::main]
async fn main() -> Result<(), AssignmentError> {
    // retrive and parse the string date into NaiveDate from the cli arg
    let (start_date, end_date) = ParsedData::parse().parse_data_to_date()?;

    // get the RpcClient object
    let client = helper::get_client();

    // fetch all signatures upto 1000 for gari token
    let signatures_all = client::get_signatures(&client).await?;

    // filter signatures according to the signature date, user input start and end date
    let signatures_filtered =
        helper::filter_data_start_end_date(signatures_all, start_date, end_date)?;

    // pass the filtered signatures to the below function
    // to fetch the transaction
    // then check if user is new
    // if user is new then print the user's data
    client::get_transactions_check_print_new_user(&client, &signatures_filtered).await?;

    Ok(())
}
