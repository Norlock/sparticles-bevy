use crate::shaders::shader::InstanceMaterialData;
use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use crate::emitters::emitter::Particle;
use crate::emitters::emitter::Velocity;

pub struct DevUIPlugin;

impl Plugin for DevUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(update_metrics)
            .add_system(update_fps_text)
            .add_system(update_frame_time_text)
            .add_system(update_particle_count_text)
            .add_plugin(FrameTimeDiagnosticsPlugin);
    }
}

struct Metric {
    fps: f64,
    frame_time: f64,
    last_updated_ms: u128,
    particle_count: usize,
}

#[derive(Component)]
struct FrameTimeText;

#[derive(Component)]
struct FPSText;

#[derive(Component)]
struct ParticleCountText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    // FPS
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(1., 1., 1.),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(1., 1., 1.),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FPSText);

    // Frame time
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Frame time: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(1., 1., 1.),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(1., 1., 1.),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(25.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FrameTimeText);

    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Particle count: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(1., 1., 1.),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(1., 1., 1.),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(45.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ParticleCountText);

    commands.insert_resource(Metric {
        frame_time: -1.,
        fps: -1.,
        last_updated_ms: time.time_since_startup().as_millis(),
        particle_count: 0,
    })
}

fn update_fps_text(mut query: Query<&mut Text, With<FPSText>>, metrics: ResMut<Metric>) {
    let mut text = query.single_mut();
    text.sections[1].value = convert(metrics.fps, 3);
}

fn update_frame_time_text(
    mut query: Query<&mut Text, With<FrameTimeText>>,
    metrics: ResMut<Metric>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = convert(metrics.frame_time, 6);
}

fn update_particle_count_text(
    mut text_query: Query<&mut Text, With<ParticleCountText>>,
    metrics: ResMut<Metric>,
) {
    let mut text = text_query.single_mut();
    text.sections[1].value = format!("{}", metrics.particle_count);
}

fn convert(val: f64, precision: usize) -> String {
    format!("{:.precision$}", val)
}

const UPDATE_METRICS_MS: u128 = 500;

fn update_metrics(
    mut metrics: ResMut<Metric>,
    particle_query: Query<&InstanceMaterialData>,
    diagnostics: ResMut<'_, Diagnostics>,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();
    let elapsed_ms = total_elapsed_ms - metrics.last_updated_ms;

    if elapsed_ms < UPDATE_METRICS_MS {
        return;
    }

    let frame_time = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .unwrap()
        .value()
        .unwrap_or(-1.);

    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .value()
        .unwrap_or(-1.);

    metrics.fps = fps;
    metrics.frame_time = frame_time;
    metrics.last_updated_ms = total_elapsed_ms;
    metrics.particle_count = particle_query.single().0.len();
}
