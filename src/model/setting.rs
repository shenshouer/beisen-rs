use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Setting {
    JobLevels(Vec<JobLevel>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobLevel {
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "NameEnUS")]
    // pub name_en_us: String,
    #[serde(rename = "OId")]
    pub oid: String,
    #[serde(rename = "Status")]
    pub status: i32,
    #[serde(rename = "StdIsDeleted")]
    pub std_is_deleted: bool,
    // #[serde(rename = "_id")]
    // pub id: String,
}
