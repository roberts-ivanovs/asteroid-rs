mod models;

use bevy::prelude::*;
// use nalgebra::*;
// use bevy::prelude::{Vec2, Vec3, Mat3};
use std::f32::consts::PI;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(update_matrices.system())
        .add_system(get_keyboard_input.system())
        .add_system(update_logical_position.system())
        .run();
}

// ------------ Types ----------------------- //

enum Collider {
    Player,
}

pub struct Player;
struct Speed(f32);
struct Angle(f32);
struct Position(Vec2);
struct Direction(Vec2);
struct Rotation(Mat3);
struct Translation(Mat3);
struct Combination(Mat3);

// .spawn((
//     Player,
//     vec![
//         Vec2::new(-1., -1.),
//         Vec2::new(0., 1.),
//         Vec2::new(1., -1.),
//         Vec2::new(0., 0.),
//         Vec2::new(-1., -1.),
//     ],
// ));

// ---------- Functions -------------- //

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        // cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // player
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(20.0, 30.0)),
            ..Default::default()
        })
        .with(Player)
        .with(Speed(0.))
        .with(Angle(0.))
        .with(Rotation(Mat3::identity()))
        .with(Translation(Mat3::identity()))
        .with(Translation(Mat3::identity()))
        .with(Position(Vec2::new(0., 0.)))
        .with(Direction(Vec2::new(0., 1.)))
        .with(Combination(Mat3::identity()))
        .with(Collider::Player);
}

fn get_keyboard_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Speed, &mut Angle)>,
) {
    for (player, mut speed, mut angle) in &mut query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            angle.0 += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            angle.0 -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            speed.0 += 1.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            speed.0 -= 1.;
        }
    }
}

fn update_logical_position(
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform, &mut Combination)>,
) {
    // if time.delta_seconds < 5. {return}
    for (player, mut real_transformation, comb) in &mut query.iter() {
        let comination = get_mat4_from_mat3(&comb.0);
        let transf_mat = real_transformation.value_mut();
        *transf_mat = comination;
    }
}

fn update_matrices(
    time: Res<Time>,
    mut query: Query<(
        &Player,
        &mut Transform,
        &mut Speed,
        &mut Angle,
        &mut Rotation,
        &mut Translation,
        &mut Combination,
        &mut Position,
        &mut Direction,
    )>,
) {
    for (
        player,
        mut real,
        speed,
        angle,
        mut rotation,
        mut translation,
        mut combination,
        mut position,
        mut direction,
    ) in &mut query.iter()
    {
        //  ------------ NEW IMPL WHICH ONLY UPDATES MATRICES ----------------
        rotation.0 = get_matrix_rotation(angle.0);

        let dir3 = rotation.0.mul_vec3(Vec3::new(0., 1., 0.));
        direction.0 = get_vec2_from_vec3(&dir3);

        let velocity =  direction.0 * speed.0 * time.delta_seconds;
        position.0 += velocity;

        println!("position.0 {}", position.0);
        translation.0 = get_matrix_translation(position.0.x(), position.0.y());
        let scale = get_matrix_scale(0.5, 1.5).mul_scalar(5.);
        println!("translation.0 {}", translation.0);
        combination.0 = rotation.0 * scale;
    }
}

// --------------- Math helpers ----------------------- //
fn get_matrix_rotation(theta: f32) -> Mat3 {
    let theta_rad = theta * PI / 180.;
    let theta_cos = theta_rad.cos();
    let theta_sin = theta_rad.sin();
    Mat3::from_cols(
        Vec3::new(theta_cos, -theta_sin, 0.),
        Vec3::new(theta_sin, theta_cos, 0.),
        Vec3::new(0., 0., 1.),
    )
}

fn get_vec2_from_vec3(dir3: &Vec3) -> Vec2 {
    dir3.truncate()
}

fn get_matrix_translation(t_x: f32, t_y: f32) -> Mat3 {
    Mat3::from_cols(
        Vec3::new(1., 0., t_x),
        Vec3::new(0., 1., t_y),
        Vec3::new(0., 0., 1.),
    )
}

fn get_matrix_scale(s_x: f32, s_y: f32) -> Mat3 {
    Mat3::from_cols(
        Vec3::new(s_x, 0., 0.),
        Vec3::new(0., s_y, 0.),
        Vec3::new(0., 0., 1.),
    )
}

fn get_vec3_from_vec2(dir2: &Vec2) -> Vec3 {
    dir2.extend(0.)
}

fn get_mat4_from_mat3(m3: &Mat3) -> Mat4 {
    Mat4::from_cols(
        m3.x_axis().extend(0.),
        m3.y_axis().extend(0.),
        m3.z_axis().extend(0.),
        Vec3::zero().extend(1.),
    )
}
