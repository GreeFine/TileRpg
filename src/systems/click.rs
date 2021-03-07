use super::player::Player;
use bevy::{input::mouse::MouseButtonInput, prelude::*, window::CursorMoved};
#[derive(Default)]
pub struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

/// This system prints out all mouse events as they come in
pub fn clicked_action_system(
    user: Res<Player>,
    mut state: Local<State>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        println!("{:?}", event);
    }

    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        println!("{:?}", event);
    }
}
