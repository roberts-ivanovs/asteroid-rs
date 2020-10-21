mod game;
mod models;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_prototype_lyon::prelude::*;

use game::helpers::*;
use game::setup::*;
use models::structs::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_resource(WindowDescriptor {
            title: "Asteroid game".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(update_matrices.system())
        .add_system(get_keyboard_input.system())
        .add_system(text_update_system.system())
        .add_system(update_logical_position.system())
        .run();
}
