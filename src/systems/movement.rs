use bevy::prelude::*;

use super::grid::TILESIZE;
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Default)]
pub struct Movable {
    pub position: Position,
    pub moved: bool,
    pub destination: Position,
    path: Vec<Direction>,
}

impl Movable {
    pub fn new(position: Position, moved: bool, destination: Position) -> Movable {
        Movable {
            position,
            moved,
            destination,
            ..Default::default()
        }
    }
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

pub fn move_system(mut players: Query<(&mut Movable, &mut Transform)>) {
    for (mut player, mut transform) in players.iter_mut() {
        if player.moved {
            player.moved = false;
            player.path = find_grid_path(&player.position, &player.destination);
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
                    player.position.x -= 1;
                    (-tilesize.x, 0.)
                }
                Direction::Right => {
                    player.position.x += 1;
                    (tilesize.x, 0.)
                }
                Direction::Up => {
                    player.position.y += 1;
                    (0., tilesize.y)
                }
                Direction::Down => {
                    player.position.y -= 1;
                    (0., -tilesize.y)
                }
            };
            transform.translation.x += x;
            transform.translation.y += y;
        }
    }
}
