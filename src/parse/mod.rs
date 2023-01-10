use std::io;

use bevy::{
    prelude::{AssetServer, Color, Commands, Component, Res, ResMut, Resource, TextBundle},
    text::{TextAlignment, TextStyle},
    ui::{PositionType, Style, UiRect, Val},
};
use regex::Regex;

use crate::Args;

use self::{
    answer::Answer,
    map::{Map, MapTile},
    piece::{Piece, PieceTile},
    player::Player,
    win::Win,
};

pub mod answer;
pub mod map;
pub mod piece;
pub mod player;
pub mod win;

lazy_static! {
    pub static ref PLAYER_REGEX: Regex = Regex::new(r"exec p([0-9]) : \[(.+)]").unwrap();
    pub static ref FIELD_SIZES_REGEX: Regex = Regex::new("Anfield ([0-9]+) ([0-9]+):").unwrap();
    pub static ref PIECE_SIZE_REGEX: Regex = Regex::new("Piece ([0-9]+) ([0-9]+):").unwrap();
    pub static ref ANSWER_REGEX: Regex =
        Regex::new(r"-> Answer \((.)\): ([0-9]+) ([0-9]+)").unwrap();
    pub static ref SEED_REGEX: Regex = Regex::new(r"seed: ([0-9]+)").unwrap();
    pub static ref SCORE_REGEX: Regex = Regex::new(r"Player([0-9]) \((.+)\): ([0-9]+)").unwrap();
    pub static ref WINNER_REGEX: Regex = Regex::new(r"Player([0-9]) won!").unwrap();
}

#[derive(Debug, Default, Clone)]
pub enum Mode {
    #[default]
    AntFieldHeader,
    AntField,
    PieceHeader,
    Piece,
    Answer,
    WaitPeriod,
}

impl Mode {
    fn next(&mut self) {
        *self = match self {
            Mode::AntFieldHeader => Mode::AntField,
            Mode::AntField => Mode::PieceHeader,
            Mode::PieceHeader => Mode::Piece,
            Mode::Piece => Mode::Answer,
            Mode::Answer => Mode::WaitPeriod,
            Mode::WaitPeriod => Mode::AntFieldHeader,
        }
    }
}

#[derive(Resource, Debug, Default, Clone, Component)]
pub struct State {
    pub finished_parsing: bool,
    pub iteration: usize,

    pub player1: Player,
    pub player2: Player,

    pub mode: Mode,

    pub map: Map,
    pub unfinished_map: Map,

    pub piece: Piece,
    pub unfinished_piece: Piece,

    pub answer: Answer,
    pub win: Win,
}

fn execute_antfield_header(i: usize, line: String, state: &mut State) {
    let caps = FIELD_SIZES_REGEX.captures(&line).unwrap();

    state.map = state.unfinished_map.clone();
    state.unfinished_map.start = i;
    state.unfinished_map.width = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    state.unfinished_map.heigth = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    state.unfinished_map.tiles = Vec::new();

    state.mode.next()
}

pub fn parse(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State>,
    args: Res<Args>,
) {
    if state.win.finished {
        return;
    }

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    if state.iteration == 0 || state.iteration == 1 {
        // player parsing
        let caps = PLAYER_REGEX.captures(&line).unwrap();

        let player = Player {
            num: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            path: caps.get(2).unwrap().as_str().to_string(),
        };

        if args.debug {
            println!("{player}");
        }

        if state.iteration == 0 {
            state.player1 = player
        } else {
            state.player2 = player
        }
    } else {
        match state.mode {
            Mode::AntFieldHeader => execute_antfield_header(state.iteration, line, &mut state),
            Mode::AntField => {
                if state.iteration - state.unfinished_map.start == 1 {
                } else {
                    // skips first map line which is useless
                    state.unfinished_map.tiles.push(
                        line.split_ascii_whitespace()
                            .nth(1)
                            .unwrap()
                            .chars()
                            .map(MapTile::from_char)
                            .collect(),
                    );

                    if state.iteration - state.unfinished_map.start - 1
                        == state.unfinished_map.heigth
                    {
                        if args.debug {
                            println!("{}", state.unfinished_map);
                        }
                        state.mode.next();
                    }
                }
            }
            Mode::PieceHeader => {
                let caps = PIECE_SIZE_REGEX.captures(&line).unwrap();

                state.piece = state.unfinished_piece.clone();
                state.unfinished_piece.start = state.iteration;
                state.unfinished_piece.width =
                    caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                state.unfinished_piece.heigth =
                    caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                state.unfinished_piece.tiles = Vec::new();

                state.mode.next();
            }
            Mode::Piece => {
                state
                    .piece
                    .tiles
                    .push(line.trim().chars().map(PieceTile::from_char).collect());

                if state.iteration - state.unfinished_piece.start == state.unfinished_piece.heigth {
                    if args.debug {
                        println!("{}", state.unfinished_piece);
                    }
                    state.mode.next();
                }
            }
            Mode::Answer => {
                state.answer = match ANSWER_REGEX.captures(&line) {
                    Some(caps) => Answer {
                        player: caps.get(1).unwrap().as_str().to_string(),
                        answer: Some((
                            caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                            caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                        )),
                    },
                    None => Answer::default(),
                };

                if args.debug {
                    println!("{}", state.answer);
                }
                state.mode.next();
            }
            Mode::WaitPeriod => {
                if FIELD_SIZES_REGEX.is_match(&line) {
                    // repeats loop
                    state.finished_parsing = true;
                    state.mode.next();
                    execute_antfield_header(state.iteration, line, &mut state)
                } else {
                    if state.win.start.is_none() && SEED_REGEX.is_match(&line) {
                        state.win.start = Some(state.iteration);
                    }

                    if let Some(i) = state.win.start {
                        match state.iteration - i {
                            0 => {
                                state.win.seed = SEED_REGEX
                                    .captures(&line)
                                    .unwrap()
                                    .get(1)
                                    .unwrap()
                                    .as_str()
                                    .to_string()
                            }
                            1 | 2 => {
                                let caps = SCORE_REGEX.captures(&line).unwrap();
                                let score = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

                                match caps.get(1).unwrap().as_str() {
                                    "1" => state.win.player1_score = score,
                                    _ => state.win.player2_score = score,
                                }
                            }
                            _ => {
                                state.win.finished = true;
                                state.win.winner = WINNER_REGEX
                                    .captures(&line)
                                    .unwrap()
                                    .get(1)
                                    .unwrap()
                                    .as_str()
                                    .parse::<usize>()
                                    .unwrap();

                                if args.debug {
                                    println!("{}", state.win);
                                }

                                let font = asset_server.load("fonts/Roboto-Regular.ttf");

                                commands.spawn(
                                    TextBundle::from_section(
                                        format!("Player {} won!", state.win.winner),
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 60.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::CENTER_LEFT)
                                    .with_style(Style {
                                        position_type: PositionType::Absolute,
                                        position: UiRect {
                                            bottom: Val::Percent(50.0),
                                            right: Val::Percent(40.0),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    }),
                                );

                                commands.spawn(
                                    TextBundle::from_section(
                                        format!(
                                            "Player 1 score: {}\nPlayer 2 score: {}",
                                            state.win.player1_score, state.win.player2_score
                                        ),
                                        TextStyle {
                                            font,
                                            font_size: 30.0,
                                            color: Color::GRAY,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::CENTER_LEFT)
                                    .with_style(Style {
                                        position_type: PositionType::Absolute,
                                        position: UiRect {
                                            bottom: Val::Percent(40.0),
                                            right: Val::Percent(43.0),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    }),
                                );
                            }
                        }
                    }
                }
            }
        };
    };

    state.iteration += 1;
}
