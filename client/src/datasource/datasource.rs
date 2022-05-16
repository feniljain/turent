use bytes::Bytes;
use std::sync::Arc;

use common::{helpers::from_rtc_ice_server, logger::Logger, models::CandidateReq};

use uuid::Uuid;
use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors, media_engine::MediaEngine, APIBuilder,
    },
    data_channel::RTCDataChannel,
    ice_transport::{
        ice_candidate::RTCIceCandidate, ice_connection_state::RTCIceConnectionState,
        ice_server::RTCIceServer,
    },
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription, RTCPeerConnection,
    },
};

use crate::{api::Api, errors::ClientError, file::File};

pub struct DataSource {
    pub id: Uuid,
    // client_id: Option<Uuid>,
    peer_connection: Arc<RTCPeerConnection>,
    logger: Logger,
}

impl DataSource {
    pub async fn new(client_api: &Api, logger: Logger) -> Result<DataSource, ClientError> {
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
            ice_servers: vec![RTCIceServer {
                urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                ..Default::default()
            }],
            ..Default::default()
        };
        // RTCIceServer {
        //     urls: vec![
        //         "turn:turn.dyte.in:443?transport=tcp".to_owned(),
        //         "turn:turn.dyte.in:3478?transport=udp".to_owned(),
        //     ],
        //     username: "dyte".to_string(),
        //     credential: "dytein".to_string(),
        //     ..Default::default()
        // },
        let ice_servers = config.ice_servers.clone();

        //Make peer connection
        let peer_connection = Arc::new(
            api.new_peer_connection(config)
                .await
                .map_err(|err| ClientError::WebRTCError(err))?,
        );

        let uuid = Uuid::new_v4();

        client_api
            .register_server(common::models::RegisterOrRefreshServerReq {
                server_id: uuid.to_string(),
                files: Some(vec![String::from("67e55044-10b1-426f-9247-bb680e5ff1b8")]),
                ice_candidates: Some(
                    ice_servers
                        .into_iter()
                        .map(|x| from_rtc_ice_server(x))
                        .collect(),
                ),
                url: String::from("http://localhost:8080"),
            })
            .await?;

        //Register on_peer_connection_state_change

        // Set the handler for Peer connection state
        // This will notify you when the peer has connected/disconnected
        peer_connection
            .on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
                println!("Peer Connection State has changed, datasource: {}", s);

                Box::pin(async {})
            }))
            .await;

        ////Register listener for onIceCandidate
        //// let pc = Arc::downgrade(&peer_connection);
        //peer_connection
        //    .on_ice_candidate(Box::new(move |c: Option<RTCIceCandidate>| {
        //        println!("on_ice_candidate datasource {:?}", c);

        //        // let pc2 = pc.clone();
        //        // if let Some(ice_candidate) = c {
        //        //     if let Some(pc) = pc2.upgrade() {
        //        //         pc.add_ice_candidate(RTCIceCandidateInit {
        //        //             candidate: ice_candidate.to_string(),
        //        //             ..Default::default()
        //        //         })
        //        //         .await;
        //        //     }
        //        // }

        //        Box::pin(async move {})
        //    }))
        //    .await;

        peer_connection
            .on_ice_connection_state_change(Box::new(|connection_state: RTCIceConnectionState| {
                println!(
                    "ICE Connection State has changed, datasource: {}",
                    connection_state
                );
                Box::pin(async {})
            }))
            .await;

        Ok(Self {
            id: uuid,
            // client_id: None,
            peer_connection,
            logger,
        })
    }

    pub async fn accept_connection_req_of_client(
        &self,
        client_id: Uuid,
        offer: RTCSessionDescription,
    ) -> Result<RTCSessionDescription, ClientError> {
        // self.client_id = Some(client_id);

        self.peer_connection
            .set_remote_description(offer)
            .await
            .map_err(|err| {
                self.logger.log_err(&err);
                ClientError::WebRTCError(err)
            })?;

        //Register listener for onIceCandidate
        self.peer_connection
            .on_ice_candidate(Box::new(move |c: Option<RTCIceCandidate>| {
                println!("on_ice_candidate datasource {:?}", c);

                let client_id = client_id.clone();
                let client_api = Api::new();

                Box::pin(async move {
                    if let Some(ice_candidate) = c {
                        match client_api
                            .send_candidate(
                                "http://127.0.0.1:8081".to_string(),
                                CandidateReq {
                                    id: client_id.to_string(),
                                    candidate: ice_candidate,
                                },
                            )
                            .await
                        {
                            Ok(_) => println!("Candidate sent from datasource"),
                            Err(err) => {
                                println!("Error sending candidate from datasource, err: {:?}", err)
                            }
                        }
                    }
                })
            }))
            .await;

        let answer = self
            .peer_connection
            .create_answer(None)
            .await
            .map_err(|err| ClientError::WebRTCError(err))?;

        // Create channel that is blocked until ICE Gathering is complete
        // let mut gather_complete = self.peer_connection.gathering_complete_promise().await;

        self.peer_connection
            .set_local_description(answer.clone())
            .await
            .map_err(|err| ClientError::WebRTCError(err))?;

        self.peer_connection
            .on_data_channel(Box::new(move |d: Arc<RTCDataChannel>| {
                let d_label = d.label().to_owned();
                let d_id = d.id();
                println!("New DataChannel {} {}", d_label, d_id);

                //====
                // Register channel opening handling

                Box::pin(async move {
                    let d2 = Arc::clone(&d);
                    let d_label2 = d_label.clone();
                    let d_id2 = d_id;

                    d.on_open(Box::new(move || {
                        println!("Data channel '{}'-'{}' open", d_label2, d_id2);
                        Box::pin(async move {
                            let file = File::new(
                                "/Users/feniljain/Projects/rust-projects/turent".to_string(),
                                "example_file.txt".to_string(),
                            );
                            match file.chunkify() {
                                Ok(chunks) => {
                                    for chunk in chunks {
                                        match d2.send(&Bytes::copy_from_slice(&chunk)).await {
                                            Ok(_) => {}
                                            Err(err) => {
                                                println!("Err sending chunk: {:?}", err);
                                            }
                                        }
                                    }

                                    match d2.close().await {
                                        Ok(_) => {}
                                        Err(err) => {
                                            println!("Error while closing file: {:?}", err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    println!("Err chunkifying: {:?}", err);
                                }
                            }
                        })
                    }))
                    .await;
                })
            }))
            .await;

        // Block until ICE Gathering is complete, disabling trickle ICE
        // we do this because we only can exchange one signaling message
        // in a production application you should exchange ICE Candidates via OnICECandidate
        // let _ = gather_complete.recv().await;
        // self.logger.log_debug("ICE Gathering complete");

        Ok(answer)
    }

    pub async fn add_ice_candidate(&self, candidate: RTCIceCandidate) -> Result<(), ClientError> {
        if let Ok(c) = candidate.to_json().await {
            return self
                .peer_connection
                .add_ice_candidate(c)
                .await
                .map_err(|err| ClientError::WebRTCError(err));
        }

        return Err(ClientError::ErrConvertingCandidateToJson);
    }

    // pub fn send_file_to_client() {}

    // pub fn disconnect_from_client() {}
}
