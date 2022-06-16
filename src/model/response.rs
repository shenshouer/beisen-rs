use crate::utils::serde::deserialize_null_default;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BeisenResponse<T, K>
where
    T: Default,
{
    #[serde(rename = "Code")]
    pub code: K,
    #[serde(rename = "Data")]
    pub data: Option<T>,
    #[serde(
        default,
        rename = "Message",
        deserialize_with = "deserialize_null_default"
    )]
    pub message: String,
    #[serde(
        default,
        rename = "Extra",
        deserialize_with = "deserialize_null_default"
    )]
    pub extra: String,
    #[serde(
        default,
        rename = "Total",
        deserialize_with = "deserialize_null_default"
    )]
    pub total: usize,
}

// impl<T> BeisenResponse<T> {
//     pub fn is_error(&self) -> bool {
//         self.code == "200"
//     }
// }
