use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use ode_bevy::{Body, Collider, ODEPlugin, StepTime};

struct FpsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: " Cubes: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: " Physics: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            vsync: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ODEPlugin { hz: 100.0 })
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_startup_system(setup.system())
        .add_system(text_update.system())
        .add_system(remove_too_low.system())
        .add_system(cube_spawner.system())
        .run();
}

fn text_update(
    diagnostics: Res<Diagnostics>,
    step_time: Option<Res<StepTime>>,
    mut q_text: Query<&mut Text, With<FpsText>>,
    q_cubes: Query<&Body>,
) {
    let ncubes = q_cubes.iter().count();
    let st = step_time.map(|x| x.0).unwrap_or(1.0);
    for mut text in q_text.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2} ({:.2} ms)", average, 1000.0 / average);
            }
        }
        text.sections[3].value = format!("{}", ncubes);
        let step = 0.01;
        text.sections[5].value = format!("{:.1}x", step / st);
    }
}

fn remove_too_low(mut commands: Commands, query: Query<(Entity, &Transform), With<Body>>) {
    for (e, t) in query.iter() {
        if t.translation.y < -1.0 {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn cube_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
) {
    let x = (rand::random::<f32>() - 1.0) * 5.0;
    let y = rand::random::<f32>() * 10.0 + 2.0;
    let z = (rand::random::<f32>() - 1.0) * 5.0;

    let r = rand::random::<f32>();
    let g = rand::random::<f32>();
    let b = rand::random::<f32>();

    if keys.pressed(KeyCode::Space) {
        commands
            .spawn()
            .insert(Body { mass: 10.0 })
            .insert(Transform::from_xyz(x, y, z))
            .insert(GlobalTransform::identity())
            .with_children(|parent| {
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(r, g, b).into()),
                        ..Default::default()
                    })
                    .insert(Collider { side_len: 1.0 });
            });
    }
}
