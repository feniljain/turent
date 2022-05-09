#[derive(Debug, Clone)]
pub enum DiscoveryError {
    ServerNotFoundError,
    InternalServerError,
}

impl std::error::Error for DiscoveryError {}

impl std::fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryError::ServerNotFoundError => write!(f, "Server not found in database"),
            DiscoveryError::InternalServerError => write!(f, "Internal Server Error"),
        }
    }
}
