use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JobPost {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OId")]
    pub oid: String,
    #[serde(rename = "OIdJobLevelType", default)]
    pub oid_job_level_type: String,
    #[serde(rename = "Status")]
    pub status: i32,
    #[serde(rename = "IsDeleted")]
    pub is_deleted: bool,
}
