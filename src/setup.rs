use bevy::prelude::{
    shape, Assets, Color, Commands, Mesh, PbrBundle, ResMut, StandardMaterial, Transform,
};
use clap::Parser;

use crate::{draw::MapBlock, Args};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut args: ResMut<Args>,
) {
    *args = Args::parse();

    for x in 0..100 {
        for y in 0..100 {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
                    transform: Transform::from_xyz(x as f32, 0.0, y as f32),
                    ..Default::default()
                },
                MapBlock { x, y },
            ));
        }
    }
}
