pub mod args;
pub mod client;
pub mod error;
pub mod helper;

use args::ParsedData;
use clap::Parser;
use error::AssignmentError;

#[tokio::main]
async fn main() -> Result<(), AssignmentError> {
    let (start_date, end_date) = ParsedData::parse().parse_data_to_date()?;

    let client = helper::get_client();

    let signatures_all = client::get_signatures(&client).await?;

    let signatures_filtered =
        helper::filter_data_start_end_date(signatures_all, start_date, end_date)?;

    client::get_transactions_check_print_new_user(&client, &signatures_filtered).await?;

    Ok(())
}
