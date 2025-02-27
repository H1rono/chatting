pub use tonic::{Code, Status};

pub use crate::error::Failure;
pub type Timestamp = chrono::DateTime<chrono::Utc>;

pub fn convert_timestamp(t: Timestamp) -> Result<prost_types::Timestamp, Failure> {
    let seconds = t.timestamp();
    let nanos = t.timestamp_subsec_nanos() as i32;
    let t = prost_types::Timestamp { seconds, nanos };
    Ok(t)
}
