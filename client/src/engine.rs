use std::sync::{Arc, Mutex};

use actix_web::{post, web, App, HttpResponse, HttpServer};
use common::{
    entities::ServerInfo,
    logger::Logger,
    models::{FindServerForFileReq, OfferReq, OfferRes},
};
use uuid::{uuid, Uuid};

use crate::{
    api::Api,
    datasink::DataSinkManager,
    datasource::DataSourceManager,
    errors::{ApiError, ClientError},
};

pub struct Engine {
    data_source_manager: Option<DataSourceManager>,
    data_sink_manager: Option<DataSinkManager>,
    api: Api,
    logger: Logger,
}

pub struct AppState {
    engine: Arc<Mutex<Engine>>,
}

impl Engine {
    pub async fn new(
        server_uuid: Option<Uuid>,
        init_data_sink: bool,
        init_data_source: bool,
    ) -> Result<Engine, ClientError> {
        let logger = Logger::new(true);

        let api = Api::new();

        if let Err(_) = api.discovery_hello().await {
            return Err(ClientError::DiscoveryServerNotUp);
        }

        let mut data_sink_manager = None;
        let mut data_source_manager = None;

        if init_data_sink {
            data_sink_manager = Some(DataSinkManager::new(logger.clone())?);
        }

        if init_data_source {
            data_source_manager = Some(DataSourceManager::new(server_uuid, logger.clone())?);
        }

        Ok(Self {
            data_source_manager,
            data_sink_manager,
            api,
            logger,
        })
    }

    pub async fn new_data_sink(
        &mut self,
        file_id: Uuid,
        server_info: ServerInfo,
    ) -> Result<(), ClientError> {
        if let Some(data_sink_manager) = &mut self.data_sink_manager {
            return data_sink_manager
                .new_data_sink(file_id, server_info, &self.api)
                .await;
        }
        Err(ClientError::InvalidConfiguration)
    }

    pub async fn new_data_source(&mut self) -> Result<(), ClientError> {
        if let Some(data_source_manager) = &mut self.data_source_manager {
            return data_source_manager.new_data_source(&self.api).await;
        }
        Err(ClientError::InvalidConfiguration)
    }

    //TODO: this method is only a temporary one, it should be remove later, and instead
    //new_data_source and new_data_sink should only be the ones used
    pub async fn start(mut self) -> Result<(), ClientError> {
        if self.data_source_manager.is_some() {
            self.new_data_source().await?;

            let app_state = web::Data::new(AppState {
                engine: Arc::new(Mutex::new(self)),
            });

            HttpServer::new(move || App::new().app_data(app_state.clone()).service(on_offer))
                .bind(("127.0.0.1", 8080))
                .map_err(|_| ClientError::ApiError(ApiError::ErrorInitializingServer))?
                .run()
                .await
                .map_err(|_| ClientError::ApiError(ApiError::ErrorInitializingServer))
        } else {
            let file_id = uuid!("67e55044-10b1-426f-9247-bb680e5ff1b8");

            // self.api.discovery_hello().await?;

            let res = self
                .api
                .find_servers(FindServerForFileReq {
                    file_id: file_id.to_string(),
                })
                .await?;

            //TODO: Add a retrying logic here which retries with next server in the list if the
            self.new_data_sink(file_id, res.servers_info[0].clone())
                .await?;

            //connection to current one fails
            // let res = self.api.send_offer(OfferReq {
            //     server_id: res.server_info.clone_into,
            //     session_desc: ,
            // });

            //TODO:
            Ok(())
        }
    }

    // pub fn get_files_list(&self, server_uuid: Uuid) -> Option<&Vec<FileType>> {
    //     self.discovery.file_lookup(server_uuid)
    // }

    // pub fn receive_file() {}
}

#[post("/on-offer")]
pub async fn on_offer(
    req: web::Json<OfferReq>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ClientError> {
    let engine = match data.engine.lock() {
        Ok(x) => x,
        Err(_) => return Err(ClientError::ApiError(ApiError::InternalServerError)),
    };

    engine.logger.log_debug("Received offer");
    let data_source_manager = match &engine.data_source_manager {
        Some(x) => x,
        None => return Err(ClientError::InvalidConfiguration),
    };
    engine.logger.log_debug("Valid config");

    let server_id = match Uuid::parse_str(&req.server_id) {
        Ok(x) => x,
        Err(_) => return Err(ClientError::ApiError(ApiError::InvalidIdFormat)),
    };
    engine.logger.log_debug("Valid server id");

    let answer = data_source_manager
        .connect_to_client(server_id, req.session_desc.clone())
        .await?;
    engine.logger.log_debug("Answer received");

    Ok(HttpResponse::Ok().json(OfferRes {
        session_desc: answer,
    }))
}
