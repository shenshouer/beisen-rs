use lazy_static::lazy_static;
use std::collections::HashMap;

pub(crate) const REQUEST_SETTING_JOB_LEVEL_COLUMNS: &[&str] = &[
    "OId",          // 对象ID
    "Name",         // 名称
    "Name_en_US",   // 名称_英文
    "StdIsDeleted", // 删除状态
    "IsDeleted",    // 是否删除
    "Status",       // 状态
    "Level",        // 级别
];
// pub(crate) const REQUEST_SETTING_UNKNOWN_COLUMNS: &[&str] = &[];

lazy_static! {
    pub(crate) static ref HASHMAP: HashMap<&'static str, &'static [&'static str]> = {
        let mut m = HashMap::new();
        m.insert("JobLevel", REQUEST_SETTING_JOB_LEVEL_COLUMNS);
        m
    };
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectName {
    JobLevel,
}

use ObjectName::*;

impl ObjectName {
    pub fn get_name(&self) -> &'static str {
        match self {
            JobLevel => "JobLevel",
            // _ => "Unknown",
        }
    }

    pub fn get_request_columns(&self) -> &'static [&'static str] {
        match self {
            JobLevel => HASHMAP[self.get_name()],
            // _ => REQUEST_SETTING_UNKNOWN_COLUMNS,
        }
    }
}
