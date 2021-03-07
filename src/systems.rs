use bevy::prelude::*;

mod animations;
mod click;
mod grid;
mod player;

pub fn init(app: &mut AppBuilder) {
    app.add_resource(player::Player::default())
        .add_startup_system(grid::spawn_grid.system())
        .add_startup_system(player::create_player.system())
        .add_system(click::clicked_action_system.system())
        .add_system(animations::animate_sprite_system.system());
}
