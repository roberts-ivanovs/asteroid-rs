use bevy::prelude::Mat3;
use bevy::prelude::Vec2;
use bevy::prelude::*;

// ------------ Types ----------------------- //
#[derive(PartialEq)]
pub enum Objects {
    Player,
    Asteroid,
    Bullet,
}

pub struct Player;
pub struct Speed(pub(crate) f32);
pub struct Angle(pub(crate) f32);
pub struct Position(pub(crate) Vec2);
pub struct Direction(pub(crate) Vec2);
pub struct Scale(pub(crate) Vec3);
pub struct Rotation(pub(crate) Mat3);
pub struct Translation(pub(crate) Mat3);
// A unit struct to help identify the FPS UI component, since there may be many Text components
pub struct FpsText;
pub struct Asteroid;
pub struct Bullet;
pub struct Flying(pub(crate) bool);
pub struct BulletSpawnerTimer(pub(crate) Timer);
pub struct AsteroidSpawnTimer(pub(crate) Timer);
