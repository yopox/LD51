use std::time::Duration;

use bevy::math::Vec2;
use bevy::sprite::TextureAtlasSprite;
use bevy_tweening::*;
use bevy_tweening::lens::*;

use crate::Transform;

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

pub(crate) fn tween_position(start: Vec2, end: Vec2, z: f32) -> Tween<Transform> {
    Tween::new(
        EaseFunction::CubicOut,
        TweeningType::Once,
        Duration::from_millis(500),
        TransformPositionLens {
            start: start.extend(z),
            end: end.extend(z),
        },
    )
}

pub(crate) static TWEEN_TIME: u64 = 500;

pub(crate) fn tween_opacity(ms: u64) -> Tween<TextureAtlasSprite> {
    Tween::new(
        EaseFunction::CubicOut,
        TweeningType::Once,
        Duration::from_millis(ms),
        TransformSpriteAlphaLens {
            start: 0.,
            end: 1.,
        }
    )
}