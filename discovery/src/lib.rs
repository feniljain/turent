// pub mod db;
// pub mod errors;
// pub mod models;

// pub struct Discovery {
//     db: Box<dyn DB>,
// }

// impl Discovery {
//     pub fn new() -> Discovery {
//         let db = MapDB::new();
//         Self { db: Box::new(db) }
//     }

//     pub fn file_lookup(&self, server_uuid: Uuid) -> Option<&Vec<FileType>> {
//         self.db.get_file_list(server_uuid)
//     }

//     // pub fn add_file(server_id: Uuid, file_id: Uuid) {}

//     pub fn add_ice_candidates() {}

//     pub fn register_or_refresh_server(
//         &mut self,
//         server_uuid: Uuid,
//         files: Option<Vec<FileType>>,
//         ice_candidates: Option<Vec<RTCIceServer>>,
//         // on_offer_cb: |offer: RTCSessionDescription| -> (),
//     ) -> Result<(), DiscoveryError> {
//         Ok(self.db.update(server_uuid, files, ice_candidates)?)
//     }

//     pub fn get_server_by_file_id(
//         &self,
//         file_id: Uuid,
//     ) -> Result<(Uuid, ServerInfo), DiscoveryError> {
//         self.db.find_server_by_file(file_id)
//     }
// }
