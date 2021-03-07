use bevy::prelude::*;

use crate::consts::{GRID_SIZE, TILE_SPACING};

const TEXTURE_PLAYER_SIZE: f32 = 24.;

#[derive(Default)]
pub struct TilesSize {
    x: f32,
    y: f32,
}

static mut TILESIZE: Option<TilesSize> = None;

pub fn create_player(
    commands: &mut Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = windows.get_primary().unwrap();
    let win_with = window.width();
    let win_height = window.height();
    let size_tile_x = (win_with - (GRID_SIZE * TILE_SPACING)) / GRID_SIZE;
    let size_tile_y = (win_height - (GRID_SIZE * TILE_SPACING)) / GRID_SIZE;
    unsafe {
        TILESIZE = Some(TilesSize {
            x: size_tile_x,
            y: size_tile_y,
        });
    }

    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TEXTURE_PLAYER_SIZE, TEXTURE_PLAYER_SIZE),
        7,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    info!("{:#?} {:#?}", size_tile_x, size_tile_y);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(size_tile_x / TEXTURE_PLAYER_SIZE)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true));
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Default)]
pub struct Player {
    positon: Vec2,
    moving: bool,
    path: Vec<Direction>,
}

pub fn move_player(mut user: &mut Player) {}
