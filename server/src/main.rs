// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::process::Command;

// use std::process::Stdio;
// use serde::{Deserialize, Serialize};
// use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection, Reply};

mod entity;
mod game;
mod ws;

// pub fn body_to_string() -> BoxedFilter<(String,)> {
//     use std::iter::FromIterator;
//     use bytes::buf::Buf;

//     warp::body::bytes();
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    static mut GAME: crate::game::Game = crate::game::Game {
        players: Vec::new(),
        projectiles: Vec::new(),
        // streams: Vec::new(),
    };

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    let posts;

    // testing
    unsafe {
        tokio::spawn(GAME.physics(0.02, clients.clone()));

        posts = warp::post()
            .and(json_body())
            .and_then(|item: Item| {
                println!("{:?}", item);
                GAME.initialize_player(item.name.to_string())
            })
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_headers(vec!["access-control-allow-origin", "content-type"])
                    .allow_methods(vec!["POST"]),
            );
    }

    // Start warp service
    println!("Starting server");
    tokio::join!(
        warp::serve(routes).run(([127, 0, 0, 1], 9001)),
        warp::serve(posts).run(([127, 0, 0, 1], 9000))
    );

    Ok(())
}

pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type Clients = Arc<Mutex<HashMap<String, Client>>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Item {
    name: String,
}

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply, Rejection> {
    println!("ws_handler");
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn json_body() -> impl Filter<Extract = (Item,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
