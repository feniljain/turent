use serde::{Deserialize, Serialize};
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

use crate::entities::{IceServer, ServerInfo};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfferRes {
    #[serde(rename = "sessionDesc")]
    pub session_desc: RTCSessionDescription,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindServerForFileReq {
    #[serde(rename = "fileId")]
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindServerForFileRes {
    #[serde(rename = "serversInfo")]
    pub servers_info: Vec<ServerInfo>,
    #[serde(rename = "success")]
    pub success: bool,
}
