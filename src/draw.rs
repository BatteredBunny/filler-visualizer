use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{
        Assets, Color, Commands, Component, Entity, Handle, Query, Res, ResMut, StandardMaterial,
    },
};
use bevy_egui::{egui, EguiContext};

use crate::parse::{map::MapTile, State};

#[derive(Component, Clone, Copy)]
pub struct MapBlock {
    pub x: usize,
    pub y: usize,
}

pub fn finished_parsing(state: Res<State>) -> ShouldRun {
    if state.finished_parsing {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn draw_info(state: Res<State>, mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Players").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("{}", state.player1));
        ui.label(format!("{}", state.player2));
    });

    egui::Window::new("Answer").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("{}", state.answer));
    });

    egui::Window::new("Piece").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("{}", state.piece));
    });
}

pub fn draw_cubes(
    mut commands: Commands,
    state: Res<State>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut cubes: Query<(Entity, &mut Handle<StandardMaterial>, &MapBlock)>,
) {
    let red = materials.add(Color::RED.into());
    let blue = materials.add(Color::BLUE.into());
    let white = materials.add(Color::WHITE.into());

    for (entity, mut material, cube) in cubes.iter_mut() {
        if !state.map.tiles.is_empty() {
            match state.map.tiles.get(cube.y) {
                Some(r) => match r.get(cube.x) {
                    Some(t) => match t {
                        MapTile::Player1 => *material = red.clone(),
                        MapTile::Player2 => *material = blue.clone(),
                        MapTile::Empty => *material = white.clone(),
                        MapTile::None => commands.entity(entity).despawn(),
                    },
                    None => commands.entity(entity).despawn(),
                },
                None => commands.entity(entity).despawn(),
            }
        } else {
            println!("This shouldn't happen")
        }
    }
}

pub fn finish_drawing(mut state: ResMut<State>) {
    state.finished_parsing = false;
}
