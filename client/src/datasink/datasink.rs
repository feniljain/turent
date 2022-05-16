use std::sync::Arc;

use common::{
    entities::{ClientInfo, ServerInfo},
    logger::Logger,
    models::{CandidateReq, OfferReq},
};
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
    pub id: Uuid,
    file_id: Uuid,
    peer_connection: Arc<RTCPeerConnection>,
    data_channel: Arc<RTCDataChannel>,
    server_info: ServerInfo,
    logger: Logger,
}

impl DataSink {
    pub async fn new(
        file_id: Uuid,
        server_info: ServerInfo,
        logger: Logger,
    ) -> Result<DataSink, ClientError> {
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

        let d1 = Arc::clone(&dc);
        dc.on_open(Box::new(move || {
        println!("Data channel '{}'-'{}' open. Random messages will now be sent to any connected DataChannels every 5 seconds", d1.label(), d1.id());

        // let d2 = Arc::clone(&d1);
        Box::pin(async move {})
    })).await;

        Ok(Self {
            id,
            file_id,
            peer_connection,
            data_channel: dc,
            server_info,
            logger,
        })
    }

    pub async fn init(&mut self, api: &Api) -> Result<(), ClientError> {
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

        //TODO: Later remove this harcoded URL
        let res = api
            .send_offer(
                self.server_info.url.clone(),
                OfferReq {
                    client_info: ClientInfo {
                        url: String::from("http://localhost:8081"),
                        id: self.id.to_string(),
                    },
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

        let server_id = self.server_info.id.clone();

        //Register listener for onIceCandidate
        self.peer_connection
            .on_ice_candidate(Box::new(move |c: Option<RTCIceCandidate>| {
                println!("on_ice_candidate datasink {:?}", c);
                let server_id = server_id.clone();
                let client_api = Api::new();

                //TODO: Remove this hardcoded URL later
                Box::pin(async move {
                    if let Some(ice_candidate) = c {
                        println!("sending req. to server id: {:?}", server_id);
                        match client_api
                            .send_candidate(
                                "http://localhost:8080".to_string(),
                                CandidateReq {
                                    id: server_id,
                                    candidate: ice_candidate,
                                },
                            )
                            .await
                        {
                            Ok(_) => println!("Candidate sent from datasink"),
                            Err(err) => {
                                println!("Error sending candidate from datasink, err: {:?}", err)
                            }
                        }
                    }
                })
            }))
            .await;

        Ok(())
    }

    pub async fn add_ice_candidate(&self, candidate: RTCIceCandidate) -> Result<(), ClientError> {
        self.peer_connection
            .add_ice_candidate(RTCIceCandidateInit {
                candidate: candidate.to_string(),
                ..Default::default()
            })
            .await
            .map_err(|err| ClientError::WebRTCError(err))
    }

    // pub fn receive_data_from_server(&self) {}

    // pub fn disconnect_from_server(&self) {}
}
