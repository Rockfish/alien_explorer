use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;

use rand::Rng;
use std::f32::consts::PI;

// use crate::camera_pan_and_orbit::PanOrbitCamera;
use crate::camera_tracking::TrackingCamera;
use crate::game_state::*;

pub fn spawn_camera(mut commands: Commands, game: Res<Game>) {
    info!("Spawning a controllable 3D perspective camera");

    let look_at = Vec3::new(
        game.player.i,
        1.,
        game.player.j
    );

    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(look_at, Vec3::Y),
            ..Default::default()
        },
        TrackingCamera {
            radius,
            ..Default::default()
        },
    ));
}

pub fn _spawn_directional_light(mut commands: Commands) {
    info!("Spawning directional 'sun' light");

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
            .into(),
        ..default()
    });
}

pub fn spawn_point_light(mut commands: Commands) {
    info!("Spawning point light");

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            range: 30.0,
            ..default()
        },
        ..default()
    });
}

pub fn spawn_game_board(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    info!("Spawning game board");

    // spawn the game board
    let tile_scene = asset_server.load("models/tile.glb#Scene0");

    game.board = (0..BOARD_SIZE_J.round() as i32)
        .map(|j| {
            (0..BOARD_SIZE_I.round() as i32)
                .map(|i| {
                    let height = rand::thread_rng().gen_range(-0.1..0.1);
                    commands.spawn(SceneBundle {
                        transform: Transform::from_xyz(i as f32, height - 0.2, j as f32),
                        scene: tile_scene.clone(),
                        ..default()
                    });
                    Cell { height }
                })
                .collect()
        })
        .collect();
}

pub fn spawn_character(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    info!("Spawning character");

    //scene: asset_server.load("models/alien.glb#Scene0"),
    // scene: asset_server.load("/Users/john/Dev_Assets/glTF-Sample-Models/2.0/CesiumMan/glTF/CesiumMan.gltf#Scene0"),

    let character_asset = asset_server.load("/Users/john/Dev_Assets/sketchfab/astronaut_game_character_animated/astro_scene.glb#Scene0");

    game.player.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game.player.i,
                        game.board[game.player.j.round() as usize][game.player.i.round() as usize].height,
                        game.player.j,
                    ),
                    rotation: Quat::from_rotation_y(-PI / 0.5),
                    ..default()
                },
                scene: character_asset,
                ..default()
            })
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::rgb(1.0, 1.0, 0.0),
                        intensity: 1000.0,
                        range: 10.0,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 2.0, 0.0),
                    ..default()
                });
            })
            .id(),
    );
}

pub fn spawn_cake(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    info!("Spawning cake");

    game.cake.handle = asset_server.load("models/cakeBirthday.glb#Scene0");

    game.cake.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform::from_xyz(
                    game.cake.i,
                    game.board[game.cake.j.round() as usize][game.cake.i.round() as usize].height + 0.2,
                    game.cake.j,
                ),
                scene: game.cake.handle.clone(),
                ..default()
            })
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::rgb(1.0, 1.0, 0.0),
                        intensity: 1000.0,
                        range: 10.0,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 2.0, 0.0),
                    ..default()
                });
            })
            .id(),
    );
}

pub fn _spawn_cake_two(
    time: Res<Time>,
    mut timer: ResMut<CakeSpawnTimer>,
    // mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
) {
    info!("Spawning random cake");

    // make sure we wait enough time before spawning the next Cake
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    if let Some(entity) = game.cake.entity {
        game.score -= 3;
        commands.entity(entity).despawn_recursive();
        game.cake.entity = None;
        // if game.score <= -5 {
        //     next_state.set(GameState::GameOver);
        //     return;
        // }
    }

    // ensure Cake doesn't spawn on the player
    loop {
        game.cake.i = rand::thread_rng().gen_range(0..BOARD_SIZE_I.round() as usize) as f32;
        game.cake.j = rand::thread_rng().gen_range(0..BOARD_SIZE_J.round() as usize) as f32;
        if game.cake.i != game.player.i || game.cake.j != game.player.j {
            break;
        }
    }
    game.cake.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform::from_xyz(
                    game.cake.i,
                    game.board[game.cake.j.round() as usize][game.cake.i.round() as usize].height + 0.2,
                    game.cake.j,
                ),
                scene: game.cake.handle.clone(),
                ..default()
            })
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::rgb(1.0, 1.0, 0.0),
                        intensity: 1000.0,
                        range: 10.0,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 2.0, 0.0),
                    ..default()
                });
            })
            .id(),
    );
}

pub fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Spawning scoreboard");

    // scoreboard
    // commands.spawn(
    //     TextBundle::from_section(
    //         "Score:",
    //         TextStyle {
    //             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //             font_size: 40.0,
    //             color: Color::rgb(0.5, 0.5, 1.0),
    //         },
    //     )
    //         .with_style(Style {
    //             position_type: PositionType::Absolute,
    //             position: UiRect {
    //                 top: Val::Px(5.0),
    //                 left: Val::Px(5.0),
    //                 ..default()
    //             },
    //             ..default()
    //         }),
    // );
    commands
        .spawn(NodeBundle {
            style: Style {
                // position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        }
        )
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                // format!("Score: {}", game.cake_eaten),
                "Something",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                },
            ));
        });
}

