use crate::{api::Api, errors::ClientError};
use common::{entities::ServerInfo, logger::Logger};
use uuid::Uuid;

use super::datasink::DataSink;

pub struct DataSinkManager {
    data_sinks: Vec<DataSink>,
    logger: Logger,
}

impl DataSinkManager {
    pub fn new(logger: Logger) -> Result<DataSinkManager, ClientError> {
        Ok(Self {
            data_sinks: vec![],
            logger,
        })
    }

    pub async fn new_data_sink(
        &mut self,
        file_id: Uuid,
        server_info: ServerInfo,
        api: &Api,
    ) -> Result<(), ClientError> {
        //Create new data sink
        let mut data_sink = DataSink::new(file_id, server_info).await?;

        data_sink.init(file_id, api).await?;

        self.data_sinks.push(data_sink);

        // let (server_id, server_info) = discovery
        //     .get_server_by_file_id(file_id)
        //     .map_err(|err| ClientError::DiscoveryError(err))?;

        //Init data sink
        // data_sink.init(file_id, server_id).await?;

        // data_sink.connect_to_server();

        //Send req to connect to server
        //Start receiving
        //Disconnect from the server
        Ok(())
    }

    // pub async fn connect_to_data_source(&self, api: &Api) -> Result<(), ClientError> {}
}
