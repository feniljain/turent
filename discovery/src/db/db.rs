use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::errors::DiscoveryError;
use common::entities::IceServer;

// pub type FileType = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub files: Vec<String>,
    pub ice_servers: Vec<IceServer>,
    pub url: String,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            files: Default::default(),
            ice_servers: Default::default(),
            url: String::new(),
        }
    }
}

pub trait DB {
    fn register(
        &mut self,
        server_uuid: String,
        files: Option<Vec<String>>,
        ice_servers: Option<Vec<IceServer>>,
    ) -> Result<(), DiscoveryError>;
    fn lookup(&self, server_uuid: String) -> bool;
    fn update(
        &mut self,
        server_uuid: String,
        files: Option<Vec<String>>,
        ice_servers: Option<Vec<IceServer>>,
        url: String,
    ) -> Result<(), DiscoveryError>;
    fn get_file_list(&self, server_uuid: String) -> Option<&Vec<String>>;
    fn get_ice_servers(&self, server_uuid: String) -> Option<&Vec<IceServer>>;
    fn find_server_by_file(&self, file_id: String) -> Result<(String, ServerInfo), DiscoveryError>;
}
