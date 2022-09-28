use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::datetime::{self, date_format};

#[derive(Debug, Deserialize, Serialize)]
pub struct JobPost {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(
        default = "datetime::default",
        rename = "CreatedTime",
        with = "date_format"
    )]
    pub created_time: DateTime<Utc>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OId")]
    pub oid: String,
    #[serde(rename = "OIdJobLevelType")]
    pub oid_job_level_type: String,
    #[serde(rename = "Status")]
    pub status: i32,
    #[serde(rename = "StdIsDeleted")]
    pub std_is_deleted: bool,
    #[serde(rename = "_id")]
    pub id: String,
}
