use bevy::prelude::Mat3;
use bevy::prelude::Vec2;
use bevy::prelude::*;

// ------------ Types ----------------------- //
pub enum Collider {
    Player,
}

pub struct Player;
pub struct Speed(pub(crate) f32);
pub struct Angle(pub(crate) f32);
pub struct Position(pub(crate) Vec2);
pub struct Direction(pub(crate) Vec2);
pub struct Rotation(pub(crate) Mat3);
pub struct Translation(pub(crate) Mat3);
// A unit struct to help identify the FPS UI component, since there may be many Text components
pub struct FpsText;
pub struct Asteroid;
pub struct AsteroidSpawnTimer(pub(crate) Timer);
