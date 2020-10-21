use bevy::{diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, prelude::*};
use core::f32::consts::PI;
use crate::models::structs::FpsText;
use crate::models::structs::Rotation;
use crate::models::structs::Position;
use crate::models::structs::Angle;
use crate::models::structs::Direction;
use crate::models::structs::Speed;

use crate::models::structs::Player;
// ---------- Functions -------------- //

pub fn get_keyboard_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Speed, &mut Angle)>,
) {
    for (player, mut speed, mut angle) in &mut query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            angle.0 -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            angle.0 += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            speed.0 += 1.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            speed.0 -= 1.;
        }
    }
}

pub fn update_logical_position(mut query: Query<(&Player, &mut Transform, &Position, &Angle)>) {
    // if time.delta_seconds < 5. {return}
    for (player, mut real, position, angle) in &mut query.iter() {
        // Calculate rotation
        let theta_rad = angle.0 * PI / 180.;
        let rot = Quat::from_axis_angle(Vec3::new(0., 0., -1.), theta_rad);

        // Set factual values
        real.set_translation(Vec3::new(position.0.x(), position.0.y(), 1.0));
        real.set_rotation(rot);
        real.set_non_uniform_scale(Vec3::new(0.5, 5.5, 1.0));
    }
}

pub fn update_matrices(
    time: Res<Time>,
    mut query: Query<(
        &mut Speed,
        &mut Angle,
        &mut Rotation,
        &mut Position,
        &mut Direction,
    )>,
) {
    for (speed, angle, mut rotation, mut position, mut direction) in &mut query.iter() {
        // Update matrix values
        // Update rotation matrix
        rotation.0 = get_matrix_rotation(angle.0);

        // Update direction matrix
        let dir3 = rotation.0.mul_vec3(Vec3::new(0., 1., 0.));
        direction.0 = get_vec2_from_vec3(&dir3);

        // Update translation matrix
        let velocity = direction.0 * speed.0 * time.delta_seconds;
        position.0 += velocity;
    }
}

// --------------- Math helpers ----------------------- //
pub fn get_matrix_rotation(theta: f32) -> Mat3 {
    let theta_rad = theta * PI / 180.;
    let theta_cos = theta_rad.cos();
    let theta_sin = theta_rad.sin();
    Mat3::from_cols(
        Vec3::new(theta_cos, -theta_sin, 0.),
        Vec3::new(theta_sin, theta_cos, 0.),
        Vec3::new(0., 0., 1.),
    )
}

pub fn get_vec2_from_vec3(dir3: &Vec3) -> Vec2 {
    dir3.truncate()
}


pub fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<(&mut Text, &FpsText)>) {
    for (mut text, _tag) in &mut query.iter() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}
