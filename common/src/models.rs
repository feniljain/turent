use serde::{Deserialize, Serialize};
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

use crate::entities::IceServer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileLookupReq {
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterOrRefreshServerReq {
    #[serde(rename = "serverId")]
    pub server_id: String,
    #[serde(rename = "files")]
    pub files: Option<Vec<String>>,
    #[serde(rename = "iceCandidates")]
    pub ice_candidates: Option<Vec<IceServer>>,
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfferReq {
    #[serde(rename = "serverId")]
    pub server_id: String,
    #[serde(rename = "sessionDesc")]
    pub session_desc: RTCSessionDescription,
}
