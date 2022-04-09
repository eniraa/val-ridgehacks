// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::process::Command;

// use std::process::Stdio;
// use serde::{Deserialize, Serialize};
// use tokio::sync::Mutex;

use futures::future::join_all;
use tokio;
use warp::Filter;
use warp::filters::BoxedFilter;
use bytes;

mod entity;
mod game;

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
        streams: Vec::new(),
    };

    let x = warp::path!("data").map(|| {});

    // testing 
    // unsafe {
    //     tokio::spawn(GAME.physics(0.02));

    //     let json = serde_json::to_string(
    //         &join_all(
    //             GAME.players
    //                 .clone()
    //                 .into_iter()
    //                 .map(|player| async move { player.clone().read().await.kinematics })
    //                 .collect::<Vec<_>>(),
    //         )
    //         .await,
    //     )?;

    //     let loci = warp::path!("loci").map(|| json.clone());
    //     let spawn = warp::path!("spawn").and(warp::post()).and_then(|x: String| )
    //     .map(|| json.clone());

    //     // do this whenever there is a request to add a player
    //     tokio::spawn(GAME.initialize_player("pipetest".to_string()));
    // }

    Ok(())
}
