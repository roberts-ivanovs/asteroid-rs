use crate::models::structs::FpsText;
use crate::models::structs::Position;
use crate::models::structs::Rotation;
use crate::models::structs::Speed;
use crate::models::structs::{Angle, Asteroid};
use crate::models::structs::{Direction, Objects};
use crate::AsteroidSpawnTimer;
use crate::Bullet;
use crate::BulletSpawnerTimer;
use crate::Scale;
use crate::{MAX_X, MAX_Y};
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_prototype_lyon::prelude::primitive;
use bevy_prototype_lyon::prelude::FillOptions;
use bevy_prototype_lyon::prelude::ShapeType;
use bevy_prototype_lyon::TessellationMode;
use core::f32::consts::PI;
use rand::prelude::*;

use crate::models::structs::Player;
// ---------- Functions -------------- //

pub fn get_keyboard_input(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut timer: ResMut<BulletSpawnerTimer>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &Player,
        &mut Speed,
        &mut Angle,
        &Position,
        &Scale,
        &Direction,
    )>,
) {
    for (player, mut speed, mut angle, position, scale, direction) in &mut query.iter() {
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

        timer.0.tick(time.delta_seconds);
        if keyboard_input.pressed(KeyCode::Space) && timer.0.finished {
            timer.0.reset();
            commands
                .spawn(primitive(
                    materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    &mut meshes,
                    ShapeType::Circle(10.),
                    TessellationMode::Fill(&FillOptions::default()),
                    position.0.extend(1.),
                ))
                .with(Bullet)
                .with(Angle(angle.0))
                .with(Position(Vec2::new(position.0.x(), position.0.y())))
                .with(Speed(1000.))
                .with(Rotation(Mat3::identity()))
                .with(Scale(Vec3::new(1., 1., 1.)))
                .with(Direction(Vec2::new(direction.0.x(), direction.0.y())))
                .with(Objects::Bullet);
        }
    }
}

pub fn update_logical_position(mut query: Query<(&mut Transform, &Position, &Angle, &Scale)>) {
    // if time.delta_seconds < 5. {return}
    for (mut real, position, angle, scale) in &mut query.iter() {
        // Calculate rotation
        let theta_rad = angle.0 * PI / 180.;
        let rot = Quat::from_axis_angle(Vec3::new(0., 0., -1.), theta_rad);

        // Set factual values
        real.set_translation(Vec3::new(position.0.x(), position.0.y(), 1.0));
        real.set_rotation(rot);
        real.set_non_uniform_scale(scale.0);
    }
}

pub fn update_matrices(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Objects,
        &mut Speed,
        &mut Angle,
        &mut Rotation,
        &mut Position,
        &mut Direction,
    )>,
) {
    for (entity, object, speed, angle, mut rotation, mut position, mut direction) in
        &mut query.iter()
    {
        // Update matrix values
        // Update rotation matrix
        rotation.0 = get_matrix_rotation(angle.0);

        // Update direction matrix
        let dir3 = rotation.0.mul_vec3(Vec3::new(0., 1., 0.));
        direction.0 = get_vec2_from_vec3(&dir3);

        // Update translation matrix
        let velocity = direction.0 * speed.0 * time.delta_seconds;
        position.0 += velocity;

        // Handle going out of borders
        let mut delete = false;
        if ((MAX_X as f32 / 2.) - position.0.x().abs()) < 0. {
            let x = -1. * position.0.x();
            position.0.set_x(x);
            delete = true;
        }
        if ((MAX_Y as f32 / 2.) - position.0.y().abs()) < 0. {
            let y = -1. * position.0.y();
            position.0.set_y(y);
            delete = true;
        }

        if delete && *object == Objects::Bullet {
            commands.despawn(entity);
        }
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

pub fn spawn_asteroid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let rand_x = rng.gen_range(-300., 300.);
    let rand_y = rng.gen_range(-300., 300.);

    let radius = 1. + rng.gen_range(0., 1.);

    let mut points = vec![];

    for t in (0..360).step_by(30) {
        let x = (radius + (t as f32).cos()) * rng.gen_range(0.5, 1.5);
        let y = (radius + (t as f32).sin()) * rng.gen_range(0.5, 1.5);
        points.push((x, y).into());
    }

    let translation_vec = Vec3::new(rand_x, rand_y, 1.0);
    let mut asteroid = primitive(
        materials.add(Color::rgb(0.6, 0.3, 0.0).into()),
        meshes,
        ShapeType::Polyline {
            points: points,
            closed: false, // required by enum variant, but it is ignored by tessellator
        },
        TessellationMode::Fill(&FillOptions::non_zero()),
        translation_vec,
    );
    asteroid.transform.set_scale(10.0);

    let angle = rng.gen_range(0, 360);
    let speed = rng.gen_range(10, 100);

    commands
        .spawn(asteroid)
        .with(Asteroid)
        .with(Angle(angle as f32))
        .with(Position(translation_vec.truncate()))
        .with(Speed(speed as f32))
        .with(Rotation(Mat3::identity()))
        .with(Scale(Vec3::new(
            rng.gen_range(10., 30.),
            rng.gen_range(10., 30.),
            rng.gen_range(10., 30.),
        )))
        .with(Direction(Vec2::new(
            rng.gen_range(1., 10.),
            rng.gen_range(1., 10.),
        )))
        .with(Objects::Asteroid);
}
