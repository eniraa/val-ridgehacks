// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::process::Command;

// use std::process::Stdio;
// use serde::{Deserialize, Serialize};
// use tokio::sync::Mutex;

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

    let data: Data = Arc::new(Mutex::new(String::new()));
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and(with_data(data.clone()))
        .and_then(ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    // testing
    unsafe {
        tokio::spawn(GAME.physics(0.02, data.clone()));

        let _ = warp::path!("spawn" / String)
            .and(warp::post())
            .and_then(|x: String| GAME.initialize_player(x.to_string()));
    }

    Ok(())
}

pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Data = Arc<Mutex<String>>;

pub async fn ws_handler(
    ws: warp::ws::Ws,
    clients: Clients,
    json: Data,
) -> Result<impl Reply, Rejection> {
    println!("ws_handler");
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients, json)))
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_data(data: Data) -> impl Filter<Extract = (Data,), Error = Infallible> + Clone {
    warp::any().map(move || data.clone())
}
