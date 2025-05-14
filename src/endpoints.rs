mod execute;

pub use execute::{ExecuteRequest, ExecuteResponse};

pub enum Endpoints {
    Execute,
}
