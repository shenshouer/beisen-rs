use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::datetime::{self, date_format};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Setting {
    JobLevels(Vec<JobLevel>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobLevel {
    #[serde(
        default = "datetime::default",
        rename = "CreatedTime",
        with = "date_format"
    )]
    created_time: DateTime<Utc>,
    #[serde(rename = "IsDeleted")]
    is_deleted: bool,
    #[serde(rename = "Level")]
    level: i32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "OId")]
    oid: String,
    #[serde(rename = "Status")]
    status: i32,
    #[serde(rename = "StdIsDeleted")]
    std_is_deleted: bool,
    #[serde(rename = "_id")]
    id: String,
}
