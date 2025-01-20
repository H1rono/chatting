pub use tonic::{Code, Status};

pub type Timestamp = chrono::DateTime<chrono::Utc>;

pub fn convert_timestamp(t: Timestamp) -> Result<prost_types::Timestamp, tonic::Status> {
    let seconds = t.timestamp();
    let nanos = t.timestamp_subsec_nanos() as i32;
    let t = prost_types::Timestamp { seconds, nanos };
    Ok(t)
}

pub trait Error: std::error::Error + Send + Sync + 'static {
    fn code(&self) -> Code;
    fn message(&self) -> String;

    fn to_status(&self) -> Status {
        let code = self.code();
        let message = self.message();
        Status::new(code, message)
    }
}
