use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::utils::datetime::{self, date_format};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    #[serde(
        default = "datetime::default",
        rename = "ActualTerminateDate",
        with = "date_format"
    )]
    pub actual_terminate_date: DateTime<Local>,
    #[serde(rename = "CreatedTime", with = "date_format")]
    pub created_time: DateTime<Local>,
    #[serde(rename = "FirstParty")]
    pub first_party: String,
    #[serde(rename = "IsDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "OId")]
    pub oid: String,
    #[serde(rename = "StdIsDeleted")]
    pub std_is_deleted: bool,
    #[serde(rename = "TerminateDate", with = "date_format")]
    pub terminate_date: DateTime<Local>,
    #[serde(rename = "UserID")]
    pub user_id: u32,
}
