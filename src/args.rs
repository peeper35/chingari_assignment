use crate::error;

use chrono::NaiveDate;
use clap::Parser;
use error::AssignmentError;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct ParsedData {
    #[arg(short, long, value_name = "START_DATE")]
    start_date: String,
    #[arg(short, long, value_name = "END_DATE")]
    end_date: String,
}

impl ParsedData {
    pub fn parse_data_to_date(&self) -> Result<(NaiveDate, NaiveDate), AssignmentError> {
        Ok((
            NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d")?,
            NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d")?,
        ))
    }
}
