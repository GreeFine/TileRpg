use super::grid;
use super::player::{Player, Position};
use bevy::{prelude::*, window::CursorMoved};
#[derive(Default)]
pub struct State {
    cursor_moved_event_reader: EventReader<CursorMoved>,
    latest_cursor_pos: Vec2,
}

/// This system prints out all mouse events as they come in
pub fn clicked_action_system(
    mut players: Query<&mut Player>,
    mut state: Local<State>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    if let Some(latest_cursor_pos) = state.cursor_moved_event_reader.latest(&cursor_moved_events) {
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
