use bevy::prelude::*;

use crate::consts::{GRID_SIZE, TILE_SPACING};

pub fn spawn_grid(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let win_with = window.width();
    let win_height = window.height();
    let size_tile_x = (win_with - (GRID_SIZE * TILE_SPACING)) / GRID_SIZE;
    let size_tile_y = (win_height - (GRID_SIZE * TILE_SPACING)) / GRID_SIZE;
    let material = materials.add(Color::rgb(0.4, 0.2, 0.4).into());
    let spritevec = Vec2::new(size_tile_x, size_tile_y);
    let pos0x = TILE_SPACING / 2. + (size_tile_x / 2.) - (win_with / 2.);
    let pos0y = TILE_SPACING / 2. + (size_tile_y / 2.) - (win_height / 2.);
    for col in 0..GRID_SIZE as u32 {
        for row in 0..GRID_SIZE as u32 {
            commands.spawn(SpriteBundle {
                material: material.clone(),
                sprite: Sprite::new(spritevec),
                transform: Transform::from_translation(Vec3::new(
                    pos0x + ((size_tile_x + TILE_SPACING) * col as f32),
                    pos0y + ((size_tile_y + TILE_SPACING) * row as f32),
                    0.,
                )),
                ..Default::default()
            });
        }
    }
}
