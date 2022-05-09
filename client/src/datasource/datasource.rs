use common::{helpers::from_rtc_ice_server, logger::Logger};

use uuid::Uuid;
use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors, media_engine::MediaEngine, APIBuilder,
    },
    ice_transport::ice_server::RTCIceServer,
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, sdp::session_description::RTCSessionDescription,
        RTCPeerConnection,
    },
};

use crate::{api::Api, errors::ClientError};

pub struct DataSource {
    pub id: Uuid,
    peer_connection: RTCPeerConnection,
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
        let ice_servers = config.ice_servers.clone();
        //Make peer connection
        let peer_connection = api
            .new_peer_connection(config)
            .await
            .map_err(|err| ClientError::WebRTCError(err))?;

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
                url: String::from("http://localhost:8001"),
            })
            .await?;

        logger.log_debug(String::from("Made a new data source!"));

        Ok(Self {
            id: uuid,
            peer_connection,
            logger,
        })
    }

    pub async fn accept_connection_req_of_client(
        &self,
        offer: RTCSessionDescription,
    ) -> Result<(), ClientError> {
        self.peer_connection
            .set_remote_description(offer)
            .await
            .map_err(|err| ClientError::WebRTCError(err))
    }

    pub fn send_file_to_client() {}

    pub fn disconnect_from_client() {}
}
