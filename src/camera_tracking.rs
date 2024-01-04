#![allow(clippy::assign_op_pattern)]
#![allow(dead_code)]

// use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::Projection;
use std::f32::consts::PI;

use crate::game_state::Game;
use bevy::window::*;

/// Tags an entity as tracking camera
#[derive(Component)]
pub struct TrackingCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for TrackingCamera {
    fn default() -> Self {
        TrackingCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn update_tracking_camera(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut ev_mouse_motion: EventReader<MouseMotion>,
    mut ev_mouse_scroll: EventReader<MouseWheel>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    input_keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut TrackingCamera, &mut Transform, &Projection)>,
    game: Res<Game>,
) {
    let Ok(primary) = primary_query.get_single() else {
        return;
    };

    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let _pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) && !input_keyboard.pressed(KeyCode::ShiftLeft) {
        for mouse_motion in ev_mouse_motion.read() {
            rotation_move += mouse_motion.delta;
        }
    } else if input_mouse.pressed(orbit_button) && input_keyboard.pressed(KeyCode::ShiftLeft) {
        // Pan only if we're not rotating at the moment
        for mouse_motion in ev_mouse_motion.read() {
            pan += mouse_motion.delta * 2.0;
        }
    }

    for mouse_wheel in ev_mouse_scroll.read() {
        scroll += mouse_wheel.y * 0.05;
    }

    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut tracking_camera, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            tracking_camera.upside_down = up.y <= 0.0;
        }

        let mut any = false;

        if rotation_move.length_squared() > 0.0 {
            any = true;

            let delta_x = {
                let delta = rotation_move.x / primary.width() * PI * 2.0;
                if tracking_camera.upside_down {
                    -delta
                } else {
                    delta
                }
            };

            let delta_y = rotation_move.y / primary.height() * PI;

            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);

            // rotate around global y axis (order of operation matters)
            transform.rotation = yaw * transform.rotation;

            // rotate around local x axis
            transform.rotation = transform.rotation * pitch;
        } else if pan.length_squared() > 0.0 {
            any = true;

            // make panning distance independent of resolution and FOV,
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(
                    (projection.fov * projection.aspect_ratio) / primary.width(),
                    projection.fov / primary.height(),
                );
            }

            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;

            // println!("camera right axes: {right}");
            // println!("camera up axes: {up}");
            let axes_sum = right + up;
            println!("camera right + up axes: {axes_sum}");

            // make panning proportional to distance away from focus point
            let translation = (right + up) * tracking_camera.radius;
            tracking_camera.focus += translation;

            // let focus = tracking_camera.focus;
            //
            // println!("camera focus: {focus}");
        } else if scroll.abs() > 0.0 {
            any = true;

            tracking_camera.radius -= scroll * tracking_camera.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            tracking_camera.radius = f32::max(tracking_camera.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);

            transform.translation = tracking_camera.focus
                + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, tracking_camera.radius));
        }

        if keyboard_input.any_pressed(vec![
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ArrowRight,
            KeyCode::ArrowLeft,
        ]) {
            let target = Vec3::new(game.player.i, 1., game.player.j);
            tracking_camera.focus = target;

            transform.rotation = look_to(target - transform.translation, Vec3::Y);

            let rot_matrix = Mat3::from_quat(transform.rotation);

            transform.translation = tracking_camera.focus
                + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, tracking_camera.radius));
        }
    }
}

pub fn look_to(direction: Vec3, up: Vec3) -> Quat {
    let back = -direction.try_normalize().unwrap_or(Vec3::NEG_Z);
    let up = up.try_normalize().unwrap_or(Vec3::Y);
    let right = up
        .cross(back)
        .try_normalize()
        .unwrap_or_else(|| up.any_orthonormal_vector());
    let up = back.cross(right);
    Quat::from_mat3(&Mat3::from_cols(right, up, back))
}
