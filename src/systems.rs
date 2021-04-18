use bevy::{core::FixedTimestep, prelude::*};

mod animations;
mod click;
mod grid;
mod movement;
mod player;
mod spells;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

pub fn init(app: &mut AppBuilder) {
    app.add_startup_system(grid::spawn_grid.system())
        .add_startup_system(player::create_player.system())
        .add_system(click::clicked_action_system.system())
        .add_system(animations::animate_sprite_system.system())
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::steps_per_second(4.0))
                .with_system(movement::move_system.system()),
        )
        .add_system(animations::animate_sprite_system.system());
}
