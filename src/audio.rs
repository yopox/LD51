use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::loading::AudioAssets;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AudioPlugin)
            .add_system(update_bgm)
            .add_event::<PlayBgmEvent>();
    }
}

#[derive(Copy, Clone)]
pub enum BGM {
    TITLE,
    CLASSIC,
    MADNESS,
    GAME_OVER,
}

impl BGM {
    fn get_handle(&self, audio_assets: Res<AudioAssets>) -> Handle<AudioSource> {
        match self {
            BGM::TITLE => audio_assets.title.clone(),
            BGM::CLASSIC => audio_assets.classic.clone(),
            BGM::MADNESS => audio_assets.madness.clone(),
            BGM::GAME_OVER => audio_assets.game_over.clone(),
        }
    }
}

pub struct PlayBgmEvent(pub BGM);

fn update_bgm(
    mut bgm_events: EventReader<PlayBgmEvent>,
    audio_assets: Option<Res<AudioAssets>>,
    audio: Res<Audio>,
) {
    if audio_assets.is_none() { return; }

    // Play BGMs
    for PlayBgmEvent(bgm) in bgm_events.iter() {
        audio.stop();
        audio.set_volume(0.6);
        audio
            .play(bgm.get_handle(audio_assets.unwrap()))
            .looped();
        break;
    }
    bgm_events.clear();
}