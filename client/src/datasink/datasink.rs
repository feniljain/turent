use std::sync::Arc;

use uuid::Uuid;
use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors, media_engine::MediaEngine, APIBuilder,
    },
    data_channel::RTCDataChannel,
    ice_transport::ice_server::RTCIceServer,
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, sdp::session_description::RTCSessionDescription,
        RTCPeerConnection,
    },
};

use crate::errors::ClientError;

pub struct DataSink {
    uuid: Uuid,
    file_id: Uuid,
    peer_connection: RTCPeerConnection,
    data_channel: Option<Arc<RTCDataChannel>>,
    offer_session_desc: Option<RTCSessionDescription>,
    server_id: Option<Uuid>,
}

impl DataSink {
    pub async fn new(file_id: Uuid) -> Result<DataSink, ClientError> {
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
        let peer_connection = api
            .new_peer_connection(config)
            .await
            .map_err(|err| ClientError::WebRTCError(err))?;

        Ok(Self {
            uuid: Uuid::new_v4(),
            file_id,
            peer_connection,
            data_channel: None,
            offer_session_desc: None,
            server_id: None,
        })
    }

    pub async fn init(&mut self, file_id: Uuid, server_id: Uuid) -> Result<(), ClientError> {
        self.data_channel = Some(
            self.peer_connection
                .create_data_channel(&format!("{}:{}", file_id, server_id), None)
                .await
                .map_err(|err| ClientError::WebRTCError(err))?,
        );

        self.server_id = Some(server_id);

        Ok(())
    }

    pub async fn connect_to_server(&mut self) -> Result<(), ClientError> {
        let offer_session_desc = self
            .peer_connection
            .create_offer(None)
            .await
            .map_err(|err| ClientError::WebRTCError(err))?;
        self.offer_session_desc = Some(offer_session_desc.clone());

        self.peer_connection
            .set_local_description(offer_session_desc)
            .await
            .map_err(|err| ClientError::WebRTCError(err))?;

        Ok(())
    }

    pub fn receive_data_from_server(&self) {}

    pub fn disconnect_from_server(&self) {}
}
