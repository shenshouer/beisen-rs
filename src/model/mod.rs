/// token 结构体
mod token;
pub use token::Token;

mod employee;
pub use employee::{Employee, EmployeeBasicInfo, EmployeeServiceInfo};

mod organization;
pub use organization::Organization;

pub mod contract;

pub mod setting;

pub mod response;

mod job_post;
pub use job_post::JobPost;
