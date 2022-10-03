use std::time::Duration;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_tweening::{Animator, Delay, Sequence};

use crate::{GameState, Labels};
use crate::loading::TextureAssets;
use crate::restaurant::ShowIngredientEvent;
use crate::tween::{tween_position, tween_sprite_opacity, TWEEN_TIME};

pub struct ChefPlugin;

impl Plugin for ChefPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Cooking)
                    .label(Labels::UI)
                    .with_system(init_chef)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Cooking)
                    .label(Labels::UI)
                    .with_system(show_chef)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Cooking)
                    .label(Labels::UI)
                    .with_system(clean_chef)
            );
    }
}

#[derive(Component)]
struct ChefUI;

const TOP_POS: Vec3 = Vec3::new(54., 148., 10.);
const WRITING_TIME: u64 = 160;

fn init_chef(
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.chef.clone(),
            sprite: Sprite {
                anchor: Anchor::TopRight,
                color: Color::rgba(1., 1., 1., 0.),
                ..Default::default()
            },
            transform: Transform::from_translation(TOP_POS),
            ..Default::default()
        })
        .insert(ChefUI);
}

fn show_chef(
    mut commands: Commands,
    mut ev_show_ingredient: EventReader<ShowIngredientEvent>,
    chef: Query<Entity, With<ChefUI>>,
) {
    if let entity = chef.single() {
        for &ShowIngredientEvent { replace, position, ingredient, timer } in ev_show_ingredient.iter() {
            if !timer { continue; }
            commands
                .entity(entity)
                .insert(Animator::new(Sequence::new([
                    tween_position(writing_pos(position, 0) + Vec2::new(-64., 0.), writing_pos(position, 0), TOP_POS.z, TWEEN_TIME * 2),
                    tween_position(writing_pos(position, 0), writing_pos(position, 1), TOP_POS.z, WRITING_TIME),
                    tween_position(writing_pos(position, 1), writing_pos(position, 2), TOP_POS.z, WRITING_TIME),
                    tween_position(writing_pos(position, 2), writing_pos(position, 3), TOP_POS.z, WRITING_TIME),
                    tween_position(writing_pos(position, 3), writing_pos(position, 4), TOP_POS.z, WRITING_TIME),
                    tween_position(writing_pos(position, 4), writing_pos(position, 4) + Vec2::new(-64., 0.), TOP_POS.z, TWEEN_TIME * 2),
                ])))
                .insert(Animator::new(
                    tween_sprite_opacity(TWEEN_TIME * 2, true).then(
                        Delay::new(Duration::from_millis(WRITING_TIME * 4)).then(
                            tween_sprite_opacity(TWEEN_TIME * 2, false)))
                ));
        }
    }
}

fn writing_pos(
    ingredient: usize,
    step: u8
) -> Vec2 {
    return TOP_POS.xy() + Vec2::new(8. * step as f32, if step % 2 == 0 { 0. } else { 6. } - 16. * ingredient as f32);
}

fn clean_chef(
    mut commands: Commands,
    entities: Query<Entity, With<ChefUI>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}