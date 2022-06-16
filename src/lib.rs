//! # beisen
//! 北森接口实现
//! http://openapi.italent.cn

/// 组织人事接口定义
/// 参考：http://openapi.italent.cn/docs/tenantbase#/Employee
mod interface;
pub use interface::*;

/// 劳动合同相关接口
mod impl_contract;
/// employee接口实现
mod impl_employee;
mod impl_job_position;
mod impl_job_post;
/// 组织架构相关接口
mod impl_organization;
mod impl_setting;
/// token 获取接口实现
mod impl_token;

/// client实现
mod client;
pub use client::Client;

/// 数据模型
mod model;
pub use model::*;

/// 数据转换对象
pub mod dto;

/// 错误定义
mod err;
pub use err::Result;

pub mod utils;
pub use utils::http::PostParameters;

// 北森token获取接口
pub(crate) const BASE_URL: &str = "https://openapi.italent.cn";
