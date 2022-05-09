use webrtc::ice_transport::{ice_credential_type::RTCIceCredentialType, ice_server::RTCIceServer};

use crate::entities::{IceCredentialType, IceServer};

pub fn from_rtc_ice_server(rtc_ice_server: RTCIceServer) -> IceServer {
    let ice_credential_type = match rtc_ice_server.credential_type {
        RTCIceCredentialType::Unspecified => IceCredentialType::Unspecified,
        RTCIceCredentialType::Password => IceCredentialType::Password,
        RTCIceCredentialType::Oauth => IceCredentialType::Oauth,
    };

    IceServer {
        urls: rtc_ice_server.urls,
        username: rtc_ice_server.username,
        credential: rtc_ice_server.credential,
        credential_type: ice_credential_type,
    }
}
