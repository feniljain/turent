use std::sync::Arc;

use common::{entities::ServerInfo, models::OfferReq};
use uuid::Uuid;
use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors, media_engine::MediaEngine, APIBuilder,
    },
    data_channel::RTCDataChannel,
    ice_transport::{
        ice_candidate::{RTCIceCandidate, RTCIceCandidateInit},
        ice_connection_state::RTCIceConnectionState,
        ice_server::RTCIceServer,
    },
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        RTCPeerConnection,
    },
};

use crate::{api::Api, errors::ClientError};

pub struct DataSink {
    id: Uuid,
    file_id: Uuid,
    peer_connection: Arc<RTCPeerConnection>,
    data_channel: Arc<RTCDataChannel>,
    server_info: ServerInfo,
}

impl DataSink {
    pub async fn new(file_id: Uuid, server_info: ServerInfo) -> Result<DataSink, ClientError> {
        let mut m = MediaEngine::default();
        m.register_default_codecs()
            .map_err(|err| ClientError::WebRTCError(err))?;

        let mut registry = Registry::new();

        registry = register_default_interceptors(registry, &mut m)
            .map_err(|err| ClientError::WebRTCError(err))?;

        let api = APIBuilder::new()
            .with_media_engine(m)
            .with_interceptor_registry(registry)
            .build();

        // Prepare the configuration
        let config = RTCConfiguration {
            ice_servers: vec![
                RTCIceServer {
                    urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                    ..Default::default()
                },
                RTCIceServer {
                    urls: vec![
                        "turn:turn.dyte.in:443?transport=tcp".to_owned(),
                        "turn:turn.dyte.in:3478?transport=udp".to_owned(),
                    ],
                    username: "dyte".to_string(),
                    credential: "dytein".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        //Make peer connection
        let peer_connection = Arc::new(
            api.new_peer_connection(config)
                .await
                .map_err(|err| ClientError::WebRTCError(err))?,
        );

        //Register on_peer_connection_state_change

        // Set the handler for Peer connection state
        // This will notify you when the peer has connected/disconnected
        peer_connection
            .on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
                println!("Peer Connection State has changed, datasink: {}", s);

                Box::pin(async {})
            }))
            .await;

        //Register listener for onIceCandidate
        let pc = Arc::downgrade(&peer_connection);
        peer_connection
            .on_ice_candidate(Box::new(move |c: Option<RTCIceCandidate>| {
                println!("on_ice_candidate datasink {:?}", c);

                let pc2 = pc.clone();
                Box::pin(async move {
                    if let Some(ice_candidate) = c {
                        if let Some(pc) = pc2.upgrade() {
                            match pc
                                .add_ice_candidate(RTCIceCandidateInit {
                                    candidate: ice_candidate.to_string(),
                                    ..Default::default()
                                })
                                .await
                            {
                                Ok(x) => println!("[datasink] ICE Candidate Added"),
                                Err(err) => println!(
                                    "ICE Candidate: Failed at adding ice candidate [datasink], Error: {:?}", err
                                ),
                            }
                        } else {
                            println!("ICE Candidate: Failed at upgrading [datasink]");
                        }
                    }
                })
            }))
            .await;

        peer_connection
            .on_ice_connection_state_change(Box::new(|connection_state: RTCIceConnectionState| {
                println!(
                    "ICE Connection State has changed, datasink: {}",
                    connection_state
                );
                Box::pin(async {})
            }))
            .await;

        let id = Uuid::new_v4();

        //Create data channel
        let dc = peer_connection
            .create_data_channel(&id.to_string(), None)
            .await
            .map_err(|err| ClientError::WebRTCError(err))?;

        Ok(Self {
            id,
            file_id,
            peer_connection,
            data_channel: dc,
            server_info,
        })
    }

    pub async fn init(&mut self, file_id: Uuid, api: &Api) -> Result<(), ClientError> {
        // let mut gather_complete = self.peer_connection.gathering_complete_promise().await;
        let offer = self
            .peer_connection
            .create_offer(None)
            .await
            .map_err(|err| ClientError::WebRTCError(err))?;

        self.peer_connection
            .set_local_description(offer.clone())
            .await
            .map_err(|err| ClientError::WebRTCError(err))?;

        // let _ = gather_complete.recv().await;

        let res = api
            .send_offer(
                self.server_info.url.clone(),
                OfferReq {
                    server_id: self.server_info.id.clone(),
                    session_desc: offer,
                },
            )
            .await?;

        // self.data_channel = Some(
        //     self.peer_connection
        //         .create_data_channel(&format!("{}:{}", file_id, self.server_info.id), None)
        //         .await
        //         .map_err(|err| ClientError::WebRTCError(err))?,
        // );

        self.peer_connection
            .set_remote_description(res.session_desc)
            .await
            .map_err(|err| ClientError::WebRTCError(err))?;

        Ok(())
    }

    // pub fn receive_data_from_server(&self) {}

    // pub fn disconnect_from_server(&self) {}
}
