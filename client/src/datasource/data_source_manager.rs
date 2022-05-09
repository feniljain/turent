use common::logger::Logger;
use uuid::Uuid;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

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
        server_id: Uuid,
        offer: RTCSessionDescription,
    ) -> Result<(), ClientError> {
        for ds in &self.data_sources {
            if ds.id == server_id {
                return ds.accept_connection_req_of_client(offer).await;
            }
        }

        Err(ClientError::ServerWithGivenIdNotFound)
    }

    pub fn check_if_file_present() -> bool {
        false
    }
}
