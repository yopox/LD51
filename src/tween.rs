use std::time::Duration;

use bevy::math::Vec2;
use bevy::sprite::TextureAtlasSprite;
use bevy_tweening::*;
use bevy_tweening::lens::*;

use crate::Transform;

pub static TWEEN_TIME: u64 = 500;

pub static EV_DELETE: u64 = 0;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformSpriteAlphaLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<TextureAtlasSprite> for TransformSpriteAlphaLens {
    fn lerp(&mut self, target: &mut TextureAtlasSprite, ratio: f32) {
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

pub fn tween_opacity(ms: u64, appear: bool) -> Tween<TextureAtlasSprite> {
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