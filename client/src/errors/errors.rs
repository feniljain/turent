#[derive(Debug)]
pub enum ClientError {
    ApiError(ApiError),
    WebRTCError(webrtc::Error),
    DiscoveryServerNotUp,
    ServerWithGivenIdNotFound,
}

impl std::error::Error for ClientError {}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ApiError(err) => write!(f, "{:?}", err),
            ClientError::WebRTCError(err) => write!(f, "{:?}", err),
            ClientError::DiscoveryServerNotUp => write!(f, "Discovery Server Not functioning"),
            ClientError::ServerWithGivenIdNotFound => write!(f, "Server with given id not found"),
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    ReqwestError(reqwest::Error),
    InvalidClientConfiguration,
    InvalidIdFormat,
    ErrorInitializingSever,
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ReqwestError(err) => write!(f, "{:?}", err),
            ApiError::InvalidClientConfiguration => {
                write!(f, "This client is not started with server configuration")
            }
            ApiError::InvalidIdFormat => write!(f, "Server id is not properly formatted"),
            ApiError::ErrorInitializingSever => write!(f, "Error Initializing Server"),
        }
    }
}
