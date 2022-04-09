use tokio::process::ChildStdin;

use serde::{Deserialize, Serialize};

use euclid::{
    default::{Point2D, Vector2D},
    Angle,
};

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct KinematicData {
    pub location: Point2D<f64>,
    pub velocity: Vector2D<f64>,
    pub acceleration: f64, // not a vector; linear acceleration can only be forwards
    pub theta: Angle<f64>,
    pub omega: Angle<f64>,
    pub alpha: f64, // not an angle, used to calculate energy costs
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerCtrl {
    pub name: Option<String>,
    pub thrust: Option<f64>,
    pub torque: Option<f64>,
    pub metal_bullet: Option<bool>,
    pub laser_bullet: Option<bool>,
}

pub struct Player {
    pub kinematics: KinematicData,
    pub name: String,
    pub energy: f64,
    pub health: f64,
    pub stdin: ChildStdin,
    pub inputs: PlayerCtrl,
}

pub struct Projectile {
    pub kinematics: KinematicData,
    pub bullet: ProjectileType,
    pub origin: Arc<RwLock<Player>>,
}

pub enum ProjectileType {
    Metal,
    Laser,
    Deactivated,
}

pub trait Kinematic {
    fn kinematics(&self) -> KinematicData;
    fn collision_radius(&self) -> f64;
    fn update(&mut self, dt: f64);
    fn collides_with(&self, other: &dyn Kinematic) -> bool {
        self.kinematics()
            .location
            .distance_to(other.kinematics().location)
            <= self.collision_radius() + other.collision_radius()
    }
}

impl Kinematic for Player {
    fn kinematics(&self) -> KinematicData {
        self.kinematics
    }

    fn collision_radius(&self) -> f64 {
        16.0
    }

    fn update(&mut self, dt: f64) {
        self.kinematics.omega += Angle::radians(self.kinematics.alpha) * dt;
        self.kinematics.theta += self.kinematics.omega * dt;

        self.kinematics.velocity +=
            Vector2D::from_angle_and_length(self.kinematics.theta, self.kinematics.acceleration)
                * dt;
        self.kinematics.location += self.kinematics.velocity * dt;
    }
}

impl Kinematic for Projectile {
    fn kinematics(&self) -> KinematicData {
        self.kinematics
    }

    fn collision_radius(&self) -> f64 {
        match self.bullet {
            ProjectileType::Metal => 2.0,
            ProjectileType::Laser => 1.0,
            ProjectileType::Deactivated => 0.0,
        }
    }

    fn update(&mut self, dt: f64) {
        self.kinematics.omega += Angle::radians(self.kinematics.alpha) * dt;
        self.kinematics.theta += self.kinematics.omega * dt;

        self.kinematics.velocity +=
            Vector2D::from_angle_and_length(self.kinematics.theta, self.kinematics.acceleration)
                * dt;
        self.kinematics.location += self.kinematics.velocity * dt;
    }
}
