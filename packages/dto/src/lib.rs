pub mod error;
pub mod topic;
pub mod users;
pub mod vote;

pub use error::*;
pub use topic::*;
pub use users::*;
pub use vote::*;

pub type Result<T> = std::result::Result<T, error::ServiceError>;
