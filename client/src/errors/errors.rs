use actix_web::{error::ResponseError, HttpResponse};

#[derive(Debug)]
pub enum ClientError {
    ApiError(ApiError),
    WebRTCError(webrtc::Error),
    DiscoveryServerNotUp,
    ServerWithGivenIdNotFound,
    InvalidConfiguration,
}

impl std::error::Error for ClientError {}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ApiError(err) => write!(f, "{:?}", err),
            ClientError::WebRTCError(err) => write!(f, "{:?}", err),
            ClientError::DiscoveryServerNotUp => write!(f, "Discovery Server Not functioning"),
            ClientError::ServerWithGivenIdNotFound => write!(f, "Server with given id not found"),
            ClientError::InvalidConfiguration => write!(f, "Invalid Configuration"),
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    ReqwestError(reqwest::Error),
    InvalidIdFormat,
    ErrorInitializingServer,
    InternalServerError,
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ReqwestError(err) => write!(f, "{:?}", err),
            ApiError::InvalidIdFormat => write!(f, "Server id is not properly formatted"),
            ApiError::ErrorInitializingServer => write!(f, "Error Initializing Server"),
            ApiError::InternalServerError => write!(f, "Internal Server Error"),
        }
    }
}

impl ResponseError for ClientError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            ClientError::ApiError(err) => match err {
                ApiError::ReqwestError(_)
                | ApiError::ErrorInitializingServer
                | ApiError::InternalServerError => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                ApiError::InvalidIdFormat => reqwest::StatusCode::BAD_REQUEST,
            },
            ClientError::WebRTCError(_)
            | ClientError::DiscoveryServerNotUp
            | ClientError::ServerWithGivenIdNotFound
            | ClientError::InvalidConfiguration => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
