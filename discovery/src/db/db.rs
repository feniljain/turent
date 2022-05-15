use anyhow::Result;

use crate::errors::DiscoveryError;
use common::entities::{IceServer, ServerInfo};

// pub type FileType = Uuid;

pub trait DB {
    fn register(
        &mut self,
        server_uuid: String,
        files: Option<Vec<String>>,
        ice_servers: Option<Vec<IceServer>>,
        url: String,
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
    fn find_servers_by_file(&self, file_id: String) -> Result<Vec<ServerInfo>, DiscoveryError>;
}
