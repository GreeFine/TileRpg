use super::grid;
use super::movement::Movable;
use bevy::{prelude::*, window::CursorMoved};
#[derive(Default)]
pub struct State {
    latest_cursor_pos: Vec2,
}

/// This system prints out all mouse events as they come in
pub fn clicked_action_system<'a>(
    mut players: Query<&mut Movable>,
    mut state: Local<State>,
    mut cursor_moved_event_reader: EventReader<'a, CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if let Some(latest_cursor_pos) = cursor_moved_event_reader.iter().last() {
        state.latest_cursor_pos = latest_cursor_pos.position;
    }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let grid_pos = grid::position_to_tile(state.latest_cursor_pos);
        info!("Grid pos of click: {:#?}", grid_pos);
        let mut player = players.iter_mut().last().unwrap();
        player.moved = true;
        player.destination = grid_pos;
    }
}
