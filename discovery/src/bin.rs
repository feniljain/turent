#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod db;
pub mod errors;

use std::sync::{Arc, Mutex};

use anyhow::{bail, Result};
use common::models::FindServerForFileRes;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use uuid::Uuid;

use db::{MapDB, DB};

pub type TDBService = Box<dyn DB + 'static + Send + Sync>;

pub struct Discovery {
    db: Arc<Mutex<TDBService>>,
}

// #[("/file/lookup?file_id")]
// pub fn file_lookup(req: Json<models::FileLookupReq>) -> Result<Json<Value>> {
//     Ok(Json(json!({
//         "success":  true,
//     })))

//     // self.db.get_file_list(server_uuid)
// }

// TODO: Implement Catchers later

#[post("/register", format = "application/json", data = "<req>")]
pub fn register_or_refresh_server(
    req: Json<common::models::RegisterOrRefreshServerReq>,
    discovery: State<Discovery>,
) -> Result<Json<Value>> {
    let discovery_data = discovery.inner();

    let mut unwrapped_data = match discovery_data.db.lock() {
        Ok(x) => x,
        Err(_) => bail!("Internal Server Error"),
    };

    unwrapped_data.register(
        req.server_id.clone(),
        req.files.clone(),
        req.ice_candidates.clone(),
        req.url.clone(),
    )?;

    Ok(Json(json!({
        "success":  true,
    })))
}

#[get("/<file_id>", format = "application/json")]
pub fn get_servers_by_file_id(
    discovery: State<Discovery>,
    file_id: String,
) -> Result<Json<FindServerForFileRes>> {
    let discovery_data = discovery.inner();

    let unwrapped_data = match discovery_data.db.lock() {
        Ok(x) => x,
        Err(_) => bail!("Internal Server Error"),
    };

    //Just to make sure correct format of uuid is sent
    let _ = match Uuid::parse_str(&file_id) {
        Ok(uuid) => uuid,
        Err(_) => bail!("Invalid ID format"),
    };

    let servers_info = unwrapped_data.find_servers_by_file(file_id.to_string())?;

    Ok(Json(FindServerForFileRes {
        servers_info,
        success: true,
    }))
}

impl Discovery {
    pub fn new() -> Discovery {
        let db = MapDB::new();
        Self {
            db: Arc::new(Mutex::new(Box::new(db))),
        }
    }

    // pub fn add_file(server_id: Uuid, file_id: Uuid) {}

    pub fn add_ice_candidates() {}
}

fn main() {
    rocket().launch();
}

#[get("/")]
fn hello() -> Result<Json<Value>> {
    Ok(Json(json!({
        "success":  true,
    })))
}

fn rocket() -> rocket::Rocket {
    let rocket = rocket::ignite();

    let discovery = Discovery::new();
    let rocket = rocket.manage(discovery);
    let rocket = rocket.mount("/", routes![hello]);
    rocket.mount(
        "/api/server",
        routes![register_or_refresh_server, get_servers_by_file_id],
    )
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
