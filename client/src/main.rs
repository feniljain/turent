#![feature(proc_macro_hygiene, decl_macro, async_closure)]

mod api;
mod datasink;
mod datasource;
mod engine;
mod errors;
// #[cfg(test)]
// mod tests;

use std::env::args;

use engine::Engine;

use errors::ClientError;
use uuid::uuid;

// use webrtc::{
//     self, data_channel::RTCDataChannel, ice_transport::ice_server::RTCIceServer,
//     peer_connection::configuration::RTCConfiguration,
// };

fn parse_value(value: &str) -> bool {
    if value == "true" {
        return true;
    } else {
        return false;
    };
}

fn parse_arg(argument: String) -> (Option<bool>, Option<bool>) {
    let arg_str_vec: Vec<&str> = argument.split("=").collect();

    let mut init_client = None;
    let mut init_server = None;

    if argument.contains("init_client") {
        init_client = Some(parse_value(arg_str_vec[1]));
    }

    if argument.contains("init_server") {
        init_server = Some(parse_value(arg_str_vec[1]));
    }

    return (init_client, init_server);
}

fn parse_args(arguments: Vec<String>) -> (bool, bool) {
    let mut init_client = false;
    let mut init_server = false;

    for arg in arguments {
        let res = parse_arg(arg);
        if let Some(init_client_res) = res.0 {
            init_client = init_client_res;
        } else if let Some(init_server_res) = res.1 {
            init_server = init_server_res;
        }
    }

    (init_client, init_server)
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let arguments: Vec<String> = args().collect();
    let init_client;
    let init_server;

    (init_client, init_server) = parse_args(arguments);

    Engine::new(
        Some(uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8")),
        init_client,
        init_server,
    )
    .await?
    .start()
    .await?;
    //

    // engine.rocket().launch();
    // engine.start_server().await?;

    //=================================

    //     let config2 = RTCConfiguration {
    //         ice_servers: vec![RTCIceServer {
    //             urls: vec!["stun:stun.l.google.com:19302".to_owned()],
    //             ..Default::default()
    //         }],
    //         ..Default::default()
    //     };

    //     // Create a new RTCPeerConnection

    //     let dc1 = peer_connection1
    //         .create_data_channel("Data_Channel_1", None)
    //         .await?;

    //     // fn onDataChannel(dc: RTCDataChannel) {}

    //     // let a: Box<dyn Future<Output = ()>> = Box::new(async {});
    //     peer_connection2
    //         .on_data_channel(Box::new(|dc: Arc<RTCDataChannel>| {
    //             print!("");
    //             Box::pin(async {})
    //         }))
    //         .await;

    //     let offer_session_desc = peer_connection1.create_offer(None).await?;

    //     peer_connection1
    //         .set_local_description(offer_session_desc.clone())
    //         .await?;

    //     peer_connection2
    //         .set_remote_description(offer_session_desc)
    //         .await?;

    //     let answer_session_desc = peer_connection2.create_answer(None).await?;

    //     peer_connection2
    //         .set_local_description(answer_session_desc.clone())
    //         .await?;

    //     peer_connection1
    //         .set_remote_description(answer_session_desc)
    //         .await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_flag_parsing() {
        let res = parse_args(vec![
            String::from("init_client=true"),
            String::from("init_server=true"),
        ]);

        assert!(res.0 && res.1);

        let res = parse_args(vec![
            String::from("init_client=false"),
            String::from("init_server=true"),
        ]);

        assert!(!res.0 && res.1);

        let res = parse_args(vec![String::from("init_server=false")]);

        assert!(res.0 && !res.1);
    }

    // fn test_creation_of_server_client_acc_to_flags() {

    // }

    use crate::parse_args;
}
