/// json反序列化时时间格式化
pub mod date_format;

use chrono::{DateTime, TimeZone, Utc};

// 默认时间值
pub fn default() -> DateTime<Utc> {
    Utc.timestamp(0, 0)
}
