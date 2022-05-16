use common::logger::Logger;
use uuid::Uuid;
use webrtc::{
    ice_transport::ice_candidate::RTCIceCandidate,
    peer_connection::sdp::session_description::RTCSessionDescription,
};

use crate::{api::Api, errors::ClientError};

use super::datasource::DataSource;

pub struct DataSourceManager {
    pub uuid: Uuid,
    data_sources: Vec<DataSource>,
    logger: Logger,
}

impl DataSourceManager {
    pub fn new(uuid: Option<Uuid>, logger: Logger) -> Result<DataSourceManager, ClientError> {
        let uuid = match uuid {
            Some(x) => x,
            None => Uuid::new_v4(),
        };

        Ok(Self {
            uuid,
            data_sources: vec![],
            logger,
        })
    }

    pub async fn new_data_source(&mut self, api: &Api) -> Result<(), ClientError> {
        //Create and init new data source
        self.data_sources
            .push(DataSource::new(api, self.logger.clone()).await?);
        Ok(())
    }

    pub async fn connect_to_client(
        &self,
        client_id: Uuid,
        server_id: Uuid,
        offer: RTCSessionDescription,
    ) -> Result<RTCSessionDescription, ClientError> {
        for ds in &self.data_sources {
            if ds.id == server_id {
                return ds.accept_connection_req_of_client(client_id, offer).await;
            }
        }

        Err(ClientError::ServerWithGivenIdNotFound)
    }

    pub async fn add_ice_candidate(
        &self,
        id: Uuid,
        candidate: RTCIceCandidate,
    ) -> Result<(), ClientError> {
        for ds in &self.data_sources {
            if ds.id == id {
                return ds.add_ice_candidate(candidate).await;
            }
        }

        Err(ClientError::ServerWithGivenIdNotFound)
    }
}
