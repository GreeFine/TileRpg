use bevy::prelude::*;

use super::{
    movement::{Movable, Position},
    player::Player,
};

enum SpellsName {}

pub struct Spell {
    spell_name: SpellsName,
    cost: u32, //TODO
}

pub struct SpellCast {
    source: Player,
    destination: Position,
}

fn cast_spell(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    destination: &Position,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(ColorMaterial::color(Color::ALICE_BLUE)),
            sprite: Sprite::new(Vec2::splat(1.)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        })
        .insert(Movable::new(
            Position { x: 0, y: 0 },
            true,
            destination.clone(),
        ));
}
