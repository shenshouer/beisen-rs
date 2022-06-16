use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::datetime::{self, date_format};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    #[serde(
        default = "datetime::default",
        rename = "ActualTerminateDate",
        with = "date_format"
    )]
    actual_terminate_date: DateTime<Utc>,
    #[serde(rename = "CreatedTime", with = "date_format")]
    created_time: DateTime<Utc>,
    #[serde(rename = "FirstParty")]
    first_party: String,
    #[serde(rename = "IsDeleted")]
    is_deleted: bool,
    #[serde(rename = "OId")]
    oid: String,
    #[serde(rename = "StdIsDeleted")]
    std_is_deleted: bool,
    #[serde(rename = "TerminateDate", with = "date_format")]
    terminate_date: DateTime<Utc>,
    #[serde(rename = "UserID")]
    user_id: u32,
}
