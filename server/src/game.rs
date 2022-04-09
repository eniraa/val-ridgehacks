use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdout, Command};
use tokio::time::{sleep, Duration};

use std::process::Stdio;
use std::sync::Arc;
use tokio::sync::RwLock;

use std::f64::consts;

use rand::Rng;

use base64;

use futures::future::join_all;

use euclid::{
    default::{Point2D, Vector2D},
    Angle,
};

use crate::entity::{
    Kinematic, KinematicData, Player, PlayerCtrl, PlayerRepr, Projectile, ProjectileType,
};
use crate::Clients;

use tokio::sync::Mutex;
use warp::ws::Message;

pub struct Game {
    pub players: Vec<Arc<RwLock<Player>>>,
    pub projectiles: Vec<Projectile>,
    // pub streams: Vec<TcpStream>, // send stuff through stream on physics update
}

impl Game {
    pub async fn initialize_player(&mut self, docker: String) -> Result<String, warp::Rejection> {
        let mut cmd = Command::new("docker");
        cmd.args(&["run", "-i", &docker])
            .stdout(Stdio::piped())
            .stdin(Stdio::piped());

        let mut child = cmd.spawn().unwrap();

        let stdin = child.stdin.take().unwrap();

        let spawnpoint: Point2D<f64> = Vector2D::from_angle_and_length(
            Angle::radians(rand::thread_rng().gen_range(0.0..consts::TAU)),
            1024.0,
        )
        .to_point();

        let player = Arc::new(RwLock::new(Player {
            kinematics: KinematicData {
                location: spawnpoint,
                velocity: Vector2D::zero(),
                acceleration: 0.0,
                theta: Angle::radians(0.0),
                omega: Angle::radians(0.0),
                alpha: 0.0,
            },
            name: "[UNKNOWN]".to_string(),
            energy: 100.0,
            health: 100.0,
            stdin,
            inputs: PlayerCtrl {
                name: None,
                thrust: None,
                torque: None,
                metal_bullet: None,
                laser_bullet: None,
            },
        }));

        self.players.push(player.clone());

        tokio::spawn(Game::process_player_input(
            player.clone(),
            child.stdout.take().unwrap(), // this needs to be seperate; we can't have the player be locked because we're reading from stdout
        ));
        Ok("Success".to_string())
    }

    async fn process_player_input(player: Arc<RwLock<Player>>, stdout: ChildStdout) {
        let mut reader = BufReader::new(stdout).lines();

        while let Some(line) = reader.next_line().await.unwrap() {
            if player.read().await.health <= 0.0 || player.read().await.energy <= 0.0 {
                break;
            }

            let payload = String::from_utf8(
                base64::decode(line.as_str()).unwrap_or("{}".as_bytes().to_vec()),
            )
            .unwrap();

            let data: PlayerCtrl =
                serde_json::from_str(&payload).unwrap_or(serde_json::from_str(&"{}").unwrap());

            if let Some(thrust) = data.thrust {
                player.write().await.kinematics.acceleration = thrust;
            }

            if let Some(torque) = data.torque {
                player.write().await.kinematics.alpha = torque;
            }

            if let Some(name) = data.clone().name {
                player.write().await.name = name;
            }

            player.write().await.inputs = data.clone();
        }
    }

    pub async fn physics(&mut self, dt: f64, clients: Clients) {
        loop {
            println!("1");
            for projectile in self.projectiles.iter_mut() {
                projectile.update(dt);

                let collision_filters = join_all(
                    (*self.players)
                        .iter()
                        .map(|player| async { player.read().await.collides_with(projectile) }),
                )
                .await;
                let collisions = (*self.players)
                    .iter()
                    .enumerate()
                    .filter(|pair| collision_filters[pair.0])
                    .map(|pair| pair.1);

                for player in collisions.clone() {
                    match &projectile.bullet {
                        ProjectileType::Metal => {
                            player.write().await.health -= 8.0;
                        }
                        ProjectileType::Laser => {
                            player.write().await.health -= 1.0;
                        }
                        _ => {}
                    }
                }

                if collisions.count() > 0 {
                    projectile.bullet = ProjectileType::Deactivated;
                }
            }

            println!("2");
            for player in self.players.iter_mut() {
                let mut p = player.write().await;
                p.update(dt);

                if p.inputs.metal_bullet.unwrap_or(false) {
                    self.projectiles.push(Projectile {
                        bullet: ProjectileType::Metal,
                        kinematics: KinematicData {
                            location: p.kinematics.location
                                + Vector2D::from_angle_and_length(
                                    p.kinematics.theta,
                                    p.kinematics.velocity.length() + 4.0,
                                ),
                            velocity: p.kinematics.velocity
                                + Vector2D::from_angle_and_length(
                                    p.kinematics.theta,
                                    p.kinematics.velocity.length() + 4.0,
                                ),
                            acceleration: 0.0,
                            theta: p.kinematics.theta,
                            omega: p.kinematics.omega,
                            alpha: 0.0,
                        },
                        origin: player.clone(),
                    });
                }

                if p.inputs.laser_bullet.unwrap_or(false) {
                    self.projectiles.push(Projectile {
                        bullet: ProjectileType::Laser,
                        kinematics: KinematicData {
                            location: p.kinematics.location
                                + Vector2D::from_angle_and_length(
                                    p.kinematics.theta,
                                    p.kinematics.velocity.length() + 16.0,
                                ),
                            velocity: p.kinematics.velocity
                                + Vector2D::from_angle_and_length(
                                    p.kinematics.theta,
                                    p.kinematics.velocity.length() + 16.0,
                                ),
                            acceleration: 0.0,
                            theta: p.kinematics.theta,
                            omega: p.kinematics.omega,
                            alpha: 0.0,
                        },
                        origin: player.clone(),
                    });
                }

                p.energy -= p.kinematics.acceleration.abs() + p.kinematics.alpha.abs() + 0.01;
            }

            let mut kinematics_list = join_all(
                self.players
                    .clone()
                    .into_iter()
                    .map(|player| async move { player.clone().read().await.kinematics })
                    .collect::<Vec<_>>(),
            )
            .await;

            println!("3");
            for player in self.players.iter_mut() {
                println!("3.1");
                let p = player.read().await;
                kinematics_list.sort_by(|a, b| {
                    a.location
                        .distance_to(p.kinematics.location)
                        .partial_cmp(&b.location.distance_to(p.kinematics.location))
                        .unwrap()
                });
                println!("3.2");
                drop(p);

                let json = serde_json::to_string(&kinematics_list).unwrap();

                let _ = &player
                    .write()
                    .await
                    .stdin
                    .write_all(format!("{}\n", base64::encode(json)).as_bytes())
                    .await;
                println!("3.3");
            }

            let json = serde_json::to_string(
                &join_all(
                    self.players
                        .clone()
                        .into_iter()
                        .map(|player| async move {
                            PlayerRepr {
                                energy: player.clone().read().await.health.clone(),
                                health: player.clone().read().await.health.clone(),
                                name: player.clone().read().await.name.clone(),
                                kinematics: player.clone().read().await.kinematics,
                            }
                        })
                        .collect::<Vec<_>>(),
                )
                .await,
            );

            println!("4");
            println!("{:?}", json);

            match json {
                Ok(data) => {
                    let locked = clients.lock().await;
                    for (_, client) in locked.iter() {
                        if let Some(sender) = &client.sender {
                            println!("Sendng stuff");
                            let _ = sender.send(Ok(Message::text(data.clone())));
                        }
                    }
                }
                Err(error) => panic!("Json deserialization did not work: {}", error),
            }

            println!("5");

            // for stream in self.streams.iter_mut() {
            //     // stream.write_all();
            // }
            sleep(Duration::from_secs_f64(dt)).await;
        }
    }
}
