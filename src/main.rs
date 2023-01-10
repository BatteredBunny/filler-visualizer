#![feature(option_result_contains)]

#[macro_use]
extern crate lazy_static;

use crate::parse::parse;
use crate::parse::State;
use crate::setup::setup;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_flycam::PlayerPlugin;
use clap::Parser;
use cursor::cursor_grab_system;
use cursor::should_grab_cursor;
use draw::draw_cubes;
use draw::draw_info;
use draw::finish_drawing;
use draw::finished_parsing;
use parse::win::game_hasnt_finished;

pub mod cursor;
pub mod draw;
pub mod parse;
pub mod setup;

/// Usage: ./game_engine -f ./map00 -p1 ./filler -p2 ./bender | cargo run
#[derive(Parser, Resource, Default, Clone, Debug, Component)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Debug prints info it recieves
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// Makes it grab cursor when you click
    #[arg(short, long, default_value_t = false)]
    grab_cursor: bool,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .init_resource::<State>()
        .init_resource::<Args>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(should_grab_cursor)
                .with_system(cursor_grab_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(game_hasnt_finished)
                .with_system(parse),
        )
        .add_system(draw_info)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(finished_parsing)
                .with_system(draw_cubes)
                .with_system(finish_drawing),
        )
        .run();
}
