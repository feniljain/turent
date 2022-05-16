use actix_web::{error::ResponseError, HttpResponse};

#[derive(Debug)]
pub enum ClientError {
    ApiError(ApiError),
    WebRTCError(webrtc::Error),
    DiscoveryServerNotUp,
    ServerWithGivenIdNotFound,
    ClientWithGivenIdNotFound,
    InvalidConfiguration,
    ErrConvertingCandidateToJson,
    ErrReadingFile(String),
    ErrWritingFile(String),
}

impl std::error::Error for ClientError {}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ApiError(err) => write!(f, "{:?}", err),
            ClientError::WebRTCError(err) => write!(f, "{:?}", err),
            ClientError::DiscoveryServerNotUp => write!(f, "Discovery Server Not functioning"),
            ClientError::ServerWithGivenIdNotFound => write!(f, "Server with given id not found"),
            ClientError::ClientWithGivenIdNotFound => write!(f, "Client with given id not found"),
            ClientError::InvalidConfiguration => write!(f, "Invalid Configuration"),
            ClientError::ErrConvertingCandidateToJson => {
                write!(
                    f,
                    "Error Converting ICE Candidate to JSON for add_ice_candidate"
                )
            }
            ClientError::ErrReadingFile(err) => write!(f, "Error reading file: {:?}", err),
            ClientError::ErrWritingFile(err) => write!(f, "Error writing file: {:?}", err),
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    ReqwestError(reqwest::Error),
    InvalidIdFormat,
    ErrorInitializingServer,
    ErrorRunningServer,
    InternalServerError,
    ErrAddIceCandidateReq,
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ReqwestError(err) => write!(f, "{:?}", err),
            ApiError::InvalidIdFormat => write!(f, "Server id is not properly formatted"),
            ApiError::ErrorInitializingServer => write!(f, "Error Initializing Server"),
            ApiError::ErrorRunningServer => write!(f, "Error Running Server"),
            ApiError::InternalServerError => write!(f, "Internal Server Error"),
            ApiError::ErrAddIceCandidateReq => write!(f, "Error add ICE candidate request"),
        }
    }
}

impl ResponseError for ClientError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            ClientError::ApiError(err) => match err {
                ApiError::ReqwestError(_)
                | ApiError::ErrorInitializingServer
                | ApiError::ErrorRunningServer
                | ApiError::ErrAddIceCandidateReq
                | ApiError::InternalServerError => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                ApiError::InvalidIdFormat => reqwest::StatusCode::BAD_REQUEST,
            },
            ClientError::WebRTCError(_)
            | ClientError::DiscoveryServerNotUp
            | ClientError::ServerWithGivenIdNotFound
            | ClientError::ClientWithGivenIdNotFound
            | ClientError::ErrConvertingCandidateToJson
            | ClientError::ErrReadingFile(_)
            | ClientError::ErrWritingFile(_)
            | ClientError::InvalidConfiguration => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
