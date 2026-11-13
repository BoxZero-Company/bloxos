
pub mod app;
pub mod lifecycle;
pub mod manager;

pub use lifecycle::AppState;

pub use manager::{
    AppDescriptor,
    AppManager,
    AppRecord,
    ManagerError,
    FIRST_APP_ID,
    MAX_APPS,
};
