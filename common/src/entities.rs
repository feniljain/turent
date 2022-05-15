use serde::{Deserialize, Serialize};
use webrtc::ice_transport::{ice_credential_type::RTCIceCredentialType, ice_server::RTCIceServer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IceCredentialType {
    Unspecified,
    Password,
    Oauth,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IceServer {
    pub urls: Vec<String>,
    pub username: String,
    pub credential: String,
    pub credential_type: IceCredentialType,
}

impl IceServer {
    pub fn to_rtc_ice_server(self) -> RTCIceServer {
        let rtc_ice_credential_type = match self.credential_type {
            IceCredentialType::Unspecified => RTCIceCredentialType::Unspecified,
            IceCredentialType::Password => RTCIceCredentialType::Password,
            IceCredentialType::Oauth => RTCIceCredentialType::Oauth,
        };

        RTCIceServer {
            urls: self.urls,
            username: self.username,
            credential: self.credential,
            credential_type: rtc_ice_credential_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub files: Vec<String>,
    pub ice_servers: Vec<IceServer>,
    pub url: String,
    pub id: String,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            files: Default::default(),
            ice_servers: Default::default(),
            url: String::new(),
            id: String::new(),
        }
    }
}
