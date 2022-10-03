use std::time::Duration;

use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::TextureAtlasSprite;
use bevy_tweening::*;
use bevy_tweening::lens::*;

use crate::{Labels, Transform};

pub const TWEEN_TIME: u64 = 500;

pub const EV_DELETE: u64 = 0;
pub const EV_CUSTOMER_WAITING_TIME_ELAPSED: u64 = 1;
pub const EV_CUSTOMER_EXITED: u64 = 2;
pub const EV_ALLOW_BUTTON_UPDATE: u64 = 3;
pub const EV_CHALK: u64 = 4;
pub const EV_NOTHING: u64 = 999;

pub struct TweenPlugin;

impl Plugin for TweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(delete_entities.after(Labels::UI));
    }
}

fn delete_entities(
    mut commands: Commands,
    mut tween_events: EventReader<TweenCompleted>,
) {
    for TweenCompleted { entity, user_data } in tween_events.iter() {
        if *user_data != EV_DELETE { continue; }
        commands.entity(*entity).despawn();
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformAtlasSpriteAlphaLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<TextureAtlasSprite> for TransformAtlasSpriteAlphaLens {
    fn lerp(&mut self, target: &mut TextureAtlasSprite, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.color.set_a(value);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformSpriteAlphaLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<Sprite> for TransformSpriteAlphaLens {
    fn lerp(&mut self, target: &mut Sprite, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.color.set_a(value);
    }
}

pub fn tween_position(start: Vec2, end: Vec2, z: f32, time: u64) -> Tween<Transform> {
    Tween::new(
        EaseFunction::CubicOut,
        TweeningType::Once,
        Duration::from_millis(time),
        TransformPositionLens {
            start: start.extend(z),
            end: end.extend(z),
        },
    )
}

pub fn tween_sprite_opacity(ms: u64, appear: bool) -> Tween<Sprite> {
    Tween::new(
        EaseFunction::CubicOut,
        TweeningType::Once,
        Duration::from_millis(ms),
        TransformSpriteAlphaLens {
            start: if appear { 0. } else { 1. },
            end: if appear { 1. } else { 0. },
        }
    )
}

pub fn tween_opacity(ms: u64, appear: bool) -> Tween<TextureAtlasSprite> {
    Tween::new(
        EaseFunction::CubicOut,
        TweeningType::Once,
        Duration::from_millis(ms),
        TransformAtlasSpriteAlphaLens {
            start: if appear { 0. } else { 1. },
            end: if appear { 1. } else { 0. },
        }
    )
}

pub fn tween_text_opacity(color: Color, ms: u64, appear: bool) -> Tween<Text> {
    Tween::new(
        EaseFunction::CubicOut,
        TweeningType::Once,
        Duration::from_millis(ms),
        TextColorLens {
            start: if appear { *color.clone().set_a(0.) } else { color },
            end: if appear { color } else { *color.clone().set_a(0.) },
            section: 0
        }
    )
}