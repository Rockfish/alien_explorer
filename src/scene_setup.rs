use std::f32::consts::PI;
use rand::Rng;
use bevy::prelude::*;
use crate::camera_pan_orbit::spawn_camera;
use crate::world::*;

pub fn scene_setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    // reset the game state
    game.cake_eaten = 0;
    game.score = 0;
    game.player.i = BOARD_SIZE_I / 2.0;
    game.player.j = BOARD_SIZE_J / 2.0;
    game.player.move_cooldown = Timer::from_seconds(0.1, TimerMode::Once);

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

    let look_at = Vec3::new(
        game.player.i,
        1.,
        game.player.j
    );

    let mut commands = spawn_camera(commands, look_at);

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

    // spawn the game character
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
                //scene: asset_server.load("models/alien.glb#Scene0"),
                scene: asset_server.load("/Users/john/Dev_Assets/sketchfab/astronaut_game_character_animated/scene.gltf#Scene0"),
                // scene: asset_server.load("/Users/john/Dev_Assets/glTF-Sample-Models/2.0/CesiumMan/glTF/CesiumMan.gltf#Scene0"),
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

    // load the scene for the Cake
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
                format!("Score: {}", game.cake_eaten),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..default()
                },
            ));
        });
}
