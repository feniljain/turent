use std::collections::HashMap;

use anyhow::Result;
use common::entities::IceServer;

use crate::errors::DiscoveryError;

use super::{ServerInfo, DB};

pub struct MapDB {
    data: HashMap<String, ServerInfo>,
}

impl DB for MapDB {
    fn register(
        &mut self,
        server_uuid: String,
        files: Option<Vec<String>>,
        ice_servers: Option<Vec<IceServer>>,
    ) -> Result<(), DiscoveryError> {
        if !self.data.contains_key(&server_uuid) {
            let mut server_info: ServerInfo = Default::default();

            if let Some(files) = files {
                server_info.files = files;
            }

            if let Some(ice_servers) = ice_servers {
                server_info.ice_servers = ice_servers;
            }

            self.data.insert(server_uuid, server_info);
        }

        Ok(())
    }

    fn lookup(&self, server_uuid: String) -> bool {
        self.data.contains_key(&server_uuid)
    }

    fn update(
        &mut self,
        server_uuid: String,
        files: Option<Vec<String>>,
        ice_servers: Option<Vec<IceServer>>,
        url: String,
    ) -> Result<(), DiscoveryError> {
        let mut server_info = ServerInfo::default();

        if let Some(files) = files {
            server_info.files = files;
        }

        if let Some(ice_servers) = ice_servers {
            server_info.ice_servers = ice_servers;
        }

        server_info.url = url;

        self.data.insert(server_uuid, server_info);

        Ok(())
    }

    fn get_file_list(&self, server_uuid: String) -> Option<&Vec<String>> {
        Some(&self.data.get(&server_uuid)?.files)
    }

    fn get_ice_servers(&self, server_uuid: String) -> Option<&Vec<IceServer>> {
        Some(&self.data.get(&server_uuid)?.ice_servers)
    }

    fn find_server_by_file(&self, file_id: String) -> Result<(String, ServerInfo), DiscoveryError> {
        for (key, value) in &self.data {
            if value.files.contains(&file_id) {
                return Ok((key.clone(), value.clone()));
            }
        }

        Err(DiscoveryError::ServerNotFoundError)
    }
}

impl MapDB {
    pub fn new() -> Self {
        MapDB {
            data: HashMap::new(),
        }
    }
}
