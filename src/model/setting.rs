use chrono::{DateTime, Local};
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
    pub created_time: DateTime<Local>,
    #[serde(rename = "IsDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OId")]
    pub oid: String,
    #[serde(rename = "Status")]
    pub status: i32,
    // #[serde(rename = "StdIsDeleted")]
    // pub std_is_deleted: bool,
    #[serde(rename = "_id")]
    pub id: String,
}
