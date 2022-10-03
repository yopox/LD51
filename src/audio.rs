use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::loading::AudioAssets;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AudioPlugin)
            .add_system(update_bgm)
            .add_event::<PlayBgmEvent>()
            .add_event::<PlaySfxEvent>()
            .add_audio_channel::<BgmChannel>()
            .add_audio_channel::<SfxChannel>();
    }
}

#[derive(Copy, Clone)]
pub enum BGM {
    Title,
    Classic,
    Madness,
    GameOver,
}

impl BGM {
    fn get_handle(&self, audio_assets: &Res<AudioAssets>) -> Handle<AudioSource> {
        match self {
            BGM::Title => audio_assets.title.clone(),
            BGM::Classic => audio_assets.classic.clone(),
            BGM::Madness => audio_assets.madness.clone(),
            BGM::GameOver => audio_assets.game_over.clone(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum SFX {
    IncorrectOrder,
    CorrectOrder,
    Bread,
    Lettuce,
    Meat,
    Sauce,
    Vegetable,
    CustomerSad,
    CustomerHappy,
    Chalk,
}

impl SFX {
    fn get_handle(&self, audio_assets: &Res<AudioAssets>) -> Handle<AudioSource> {
        match self {
            SFX::IncorrectOrder => audio_assets.incorrect_order.clone(),
            SFX::CorrectOrder => audio_assets.correct_order.clone(),
            SFX::Bread => audio_assets.bread.clone(),
            SFX::Lettuce => audio_assets.lettuce.clone(),
            SFX::Meat => audio_assets.meat.clone(),
            SFX::Sauce => audio_assets.sauce.clone(),
            SFX::Vegetable => audio_assets.vegetable.clone(),
            SFX::CustomerSad => audio_assets.customer_sad.clone(),
            SFX::CustomerHappy => audio_assets.customer_happy.clone(),
            SFX::Chalk => audio_assets.chalk.clone(),
        }
    }
}

pub struct BgmChannel;

pub struct SfxChannel;

pub struct PlayBgmEvent(pub BGM);

pub struct PlaySfxEvent(pub SFX);

fn update_bgm(
    mut bgm_events: EventReader<PlayBgmEvent>,
    mut sfx_events: EventReader<PlaySfxEvent>,
    audio_assets: Option<Res<AudioAssets>>,
    bgm_channel: Res<AudioChannel<BgmChannel>>,
    sfx_channel: Res<AudioChannel<SfxChannel>>,
) {
    if audio_assets.is_none() { return; }

    // Play BGMs
    for PlayBgmEvent(bgm) in bgm_events.iter() {
        bgm_channel.stop();
        bgm_channel.set_volume(0.6);
        bgm_channel
            .play(bgm.get_handle(&audio_assets.as_ref().unwrap()))
            .looped();
        break;
    }
    bgm_events.clear();

    // Play SFXs
    for PlaySfxEvent(sfx) in sfx_events.iter() {
        sfx_channel.set_volume(0.3);
        sfx_channel.play(sfx.get_handle(&audio_assets.as_ref().unwrap()));
    }
}