/// json反序列化时时间格式化
pub mod date_format;

use chrono::{DateTime, Local, TimeZone};

// 默认时间值
pub fn default() -> DateTime<Local> {
    Local.timestamp(0, 0)
}
