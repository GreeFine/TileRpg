use super::movement::Movable;
use crate::consts::GRID_SIZE;
use bevy::prelude::*;

const TEXTURE_PLAYER_SIZE: f32 = 24.;

#[derive(Default)]
pub struct Player;

pub fn create_player(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = windows.get_primary().unwrap();
    let win_with = window.width();
    let win_height = window.height();
    let size_tile_x = win_with / GRID_SIZE;
    let size_tile_y = win_height / GRID_SIZE;

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
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(
                    size_tile_x / TEXTURE_PLAYER_SIZE,
                    (size_tile_y / TEXTURE_PLAYER_SIZE) * 1.5,
                    1.,
                ),
                translation: Vec3::new(
                    (-win_with + size_tile_x) / 2.,
                    (-win_height + TEXTURE_PLAYER_SIZE + size_tile_y) / 2.,
                    0.,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Movable::default())
        .insert(Player::default())
        .insert(Timer::from_seconds(0.1, true));
}
