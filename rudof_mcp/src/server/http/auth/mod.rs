pub mod config;
pub mod validation;
pub mod middleware;
pub mod metadata;

pub use config::AuthConfig;
pub use validation::TokenClaims;
pub use middleware::authorization_guard;
pub use metadata::protected_resource_metadata_handler;
