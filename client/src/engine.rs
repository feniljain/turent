use common::logger::Logger;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use tokio::task;
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
            let d_sink_manager = DataSinkManager::new(logger.clone())?;
            d_sink_manager
                .new_data_sink(uuid!("67e55044-10b1-426f-9247-bb680e5ff1b8"))
                .await?;
            data_sink_manager = Some(d_sink_manager);
        }

        if init_data_source {
            let mut d_source_manager = DataSourceManager::new(server_uuid, logger.clone())?;
            d_source_manager.new_data_source(&api).await?;
            data_source_manager = Some(d_source_manager);
        }

        Ok(Self {
            data_source_manager,
            data_sink_manager,
            api,
            logger,
        })
    }

    pub fn rocket(self) -> rocket::Rocket {
        let rocket = rocket::ignite();

        let rocket = rocket.manage(self);
        rocket.mount("/", routes![on_offer])
    }

    // pub fn get_files_list(&self, server_uuid: Uuid) -> Option<&Vec<FileType>> {
    //     self.discovery.file_lookup(server_uuid)
    // }

    pub fn receive_file() {}
}

#[post("/on-offer", format = "application/json", data = "<req>")]
pub fn on_offer(
    req: Json<common::models::OfferReq>,
    engine: State<Engine>,
) -> anyhow::Result<Json<Value>, ApiError> {
    let data_source_manager = match &engine.data_source_manager {
        Some(x) => x,
        None => return Err(ApiError::InvalidClientConfiguration),
    };

    let server_id = match Uuid::parse_str(&req.server_id) {
        Ok(x) => x,
        Err(_) => return Err(ApiError::InvalidIdFormat),
    };

    task::spawn_blocking(async move || {
        data_source_manager
            .connect_to_client(server_id, req.session_desc.clone())
            .await;
    });

    // .await
    // .map_err(|err| ApiError::ErrorInitializingSever);

    Ok(Json(json!({
        "success":  true,
    })))
}
