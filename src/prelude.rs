pub use tonic::{Code, Status};

pub type Timestamp = chrono::DateTime<chrono::Utc>;

pub trait Error: std::error::Error + Send + Sync + 'static {
    fn code(&self) -> Code;
    fn message(&self) -> String;

    fn to_status(&self) -> Status {
        let code = self.code();
        let message = self.message();
        Status::new(code, message)
    }
}
