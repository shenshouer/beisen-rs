use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::datetime::{self, date_format};

#[derive(Debug, Deserialize, Serialize)]
pub struct JobPost {
    #[serde(rename = "Code")]
    code: String,
    #[serde(
        default = "datetime::default",
        rename = "CreatedTime",
        with = "date_format"
    )]
    created_time: DateTime<Utc>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "OId")]
    oid: String,
    #[serde(rename = "OIdJobLevelType")]
    oid_job_level_type: String,
    #[serde(rename = "Status")]
    status: i32,
    #[serde(rename = "StdIsDeleted")]
    std_is_deleted: bool,
    #[serde(rename = "_id")]
    id: String,
}
