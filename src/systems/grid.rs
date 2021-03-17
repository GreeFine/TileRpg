use bevy::prelude::*;

use crate::consts::GRID_SIZE;

use super::movement::Position;

#[derive(Default)]
pub struct TilesSize {
    pub x: f32,
    pub y: f32,
}
pub static mut TILESIZE: Option<TilesSize> = None;

pub fn spawn_grid(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("textures/rpg/tiles/generic-rpg-tile58.png");

    let window = windows.get_primary().unwrap();
    let win_with = window.width();
    let win_height = window.height();
    let size_tile_x = win_with / GRID_SIZE;
    let size_tile_y = win_height / GRID_SIZE;
    unsafe {
        TILESIZE = Some(TilesSize {
            x: size_tile_x,
            y: size_tile_y,
        });
    }

    let material = materials.add(texture_handle.into());
    let spritevec = Vec2::new(size_tile_x, size_tile_y);
    let pos0x = (size_tile_x / 2.) - (win_with / 2.);
    let pos0y = (size_tile_y / 2.) - (win_height / 2.);
    for col in 0..GRID_SIZE as u32 {
        for row in 0..GRID_SIZE as u32 {
            commands.spawn(SpriteBundle {
                material: material.clone(),
                sprite: Sprite::new(spritevec),
                transform: Transform::from_translation(Vec3::new(
                    pos0x + (size_tile_x * col as f32),
                    pos0y + (size_tile_y * row as f32),
                    0.,
                )),
                ..Default::default()
            });
        }
    }
}

pub fn position_to_tile(positon: Vec2) -> Position {
    let tilesize;
    unsafe { tilesize = TILESIZE.as_ref().unwrap() };
    info!(
        "debug: x{} {} | y{} {}",
        positon.x,
        positon.x / tilesize.x,
        positon.y,
        positon.y / tilesize.y
    );
    Position {
        x: (positon.x / tilesize.x) as u32,
        y: (positon.y / tilesize.y) as u32,
    }
}
