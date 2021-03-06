use crate::game::helpers::spawn_asteroid;
use crate::models::structs::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    for _ in 0..10 {
        spawn_asteroid(&mut commands, &mut meshes, &mut materials);
    }

    commands
        // cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(primitive(
            materials.add(Color::rgb(76. / 255., 153. / 255., 0. / 255.).into()),
            &mut meshes,
            ShapeType::Polyline {
                points: vec![
                    (-20.0, -10.0).into(),
                    (0.0, 10.0).into(),
                    (20.0, -10.0).into(),
                    (0.0, -0.0).into(),
                    (-20.0, -10.0).into(),
                ],
                closed: false, // required by enum variant, but it is ignored by tessellator
            },
            TessellationMode::Fill(&FillOptions::default()),
            Vec3::new(-0.0, 0.0, 0.0),
        ))
        .with(Player)
        .with(Speed(0.))
        .with(Angle(0.))
        .with(Rotation(Mat3::identity()))
        .with(Translation(Mat3::identity()))
        .with(Position(Vec2::new(0., 0.)))
        .with(Direction(Vec2::new(0., 1.)))
        .with(Scale(Vec3::new(0.5, 5.5, 1.0)))
        .with(Objects::Player)
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                },
            },
            ..Default::default()
        })
        .with(FpsText);
}
