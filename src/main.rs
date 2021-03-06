mod game;
mod models;

use bevy::{app::ScheduleRunnerPlugin, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_prototype_lyon::prelude::*;
use core::time::Duration;

use game::helpers::*;
use game::setup::*;
use models::structs::*;

pub static MAX_X: u32 = 1280;
pub static MAX_Y: u32 = 720;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_resource(WindowDescriptor {
            title: "Asteroid game".to_string(),
            vsync: true,
            resizable: false,
            width: MAX_X,
            height: MAX_Y,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(
            229. / 255.,
            204. / 255.,
            255. / 255.,
        )))
        .add_startup_system(setup.system())
        .add_system(update_matrices.system())
        .add_system(get_keyboard_input.system())
        .add_system(text_update_system.system())
        .add_system(update_logical_position.system())
        // .add_system(float_asteroids.system())
        .add_resource(BulletSpawnerTimer(Timer {
            elapsed: 0.0,
            duration: Duration::from_secs(1).as_secs_f32(),
            finished: true,
            just_finished: true,
            repeating: true,
        }))
        // .add_system(spawn_shootable.system())
        .run();
}
