use std::time::Duration;

use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::TextureAtlasSprite;
use bevy_tweening::*;
use bevy_tweening::lens::*;

use crate::Transform;

pub const TWEEN_TIME: u64 = 500;

pub const EV_DELETE: u64 = 0;
pub const EV_CUSTOMER_ARRIVED: u64 = 1;
pub const EV_CUSTOMER_WAITING_TIME_ELAPSED: u64 = 2;

pub struct TweenPlugin;

impl Plugin for TweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(delete_entities);
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

#[derive(Component)]
pub struct DummyComponent;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DummyLens;

#[allow(unused_variables)]
impl Lens<DummyComponent> for DummyLens {
    fn lerp(&mut self, target: &mut DummyComponent, ratio: f32) {}
}

pub fn fake_tween() -> Tween<DummyComponent> {
    Tween::new(
        EaseFunction::CubicOut,
        TweeningType::Once,
        Duration::from_millis(1),
        DummyLens
    )
}

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