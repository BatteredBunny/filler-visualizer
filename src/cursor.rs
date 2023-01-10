use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{Input, KeyCode, MouseButton, Res, ResMut},
    window::{CursorGrabMode, Windows},
};

use crate::Args;

pub fn should_grab_cursor(args: Res<Args>) -> ShouldRun {
    if args.grab_cursor {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_grab_mode(CursorGrabMode::Confined);
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
    }
}
