use super::grid::TILESIZE;
use crate::consts::GRID_SIZE;
use bevy::prelude::*;

const TEXTURE_PLAYER_SIZE: f32 = 24.;

pub fn create_player(
    commands: &mut Commands,
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
        .spawn(SpriteSheetBundle {
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
        .with(Player::default())
        .with(Timer::from_seconds(0.1, true));
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Default)]
pub struct Player {
    pub positon: Position,
    pub moved: bool,
    pub destination: Position,
    path: Vec<Direction>,
}

pub fn find_grid_path(position: &Position, destination: &Position) -> Vec<Direction> {
    let mut result: Vec<Direction> = Vec::new();
    let mut current_x = position.x as i32;
    let mut current_y = position.y as i32;
    let destination_x = destination.x as i32;
    let destination_y = destination.y as i32;
    info!(
        "find_path: {}, {} |dest: {} {}",
        current_x, current_y, destination_x, destination_y
    );
    loop {
        if destination_x != current_x {
            if destination_x - current_x > 0 {
                result.push(Direction::Right);
                current_x += 1;
            } else {
                result.push(Direction::Left);
                current_x -= 1;
            }
        }
        if destination_y != current_y {
            if destination_y - current_y > 0 {
                result.push(Direction::Up);
                current_y += 1;
            } else {
                result.push(Direction::Down);
                current_y -= 1;
            }
        }
        if destination_y == current_y && destination_x == current_x {
            break;
        }
    }
    info!("Path to follow?: {:?}", result);
    result
}

pub fn move_player_system(mut players: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in players.iter_mut() {
        if player.moved {
            player.moved = false;
            player.path = find_grid_path(&player.positon, &player.destination);
        }

        if !player.path.is_empty() {
            let tilesize;
            unsafe {
                if TILESIZE.is_none() {
                    return;
                }
                tilesize = TILESIZE.as_ref().unwrap();
            }

            let path = player.path.remove(0);
            let (x, y) = match path {
                Direction::Left => {
                    player.positon.x -= 1;
                    (-tilesize.x, 0.)
                }
                Direction::Right => {
                    player.positon.x += 1;
                    (tilesize.x, 0.)
                }
                Direction::Up => {
                    player.positon.y += 1;
                    (0., tilesize.y)
                }
                Direction::Down => {
                    player.positon.y -= 1;
                    (0., -tilesize.y)
                }
            };
            transform.translation.x += x;
            transform.translation.y += y;
        }
    }
}
