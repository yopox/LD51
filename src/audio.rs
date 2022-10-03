use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::loading::AudioAssets;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AudioPlugin)
            .add_startup_system(init_bgm)
            .add_system(update_bgm)
            .add_event::<PlayBgmEvent>();
    }
}

#[derive(Copy, Clone)]
pub enum BGM {
    TITLE,
    CLASSIC,
    MADNESS,
}

impl BGM {
    fn get_handle(&self, audio_assets: Res<AudioAssets>) -> Handle<AudioSource> {
        match self {
            BGM::TITLE => audio_assets.title.clone(),
            BGM::CLASSIC => audio_assets.classic.clone(),
            BGM::MADNESS => audio_assets.madness.clone(),
        }
    }
}

pub struct PlayBgmEvent(pub BGM);

#[derive(Component)]
pub struct MusicVolume {
    pub volume: f64,
}

fn init_bgm(
    mut commands: Commands,
) {
    commands
        .spawn()
        .insert(MusicVolume { volume: 0.6 });
}

fn update_bgm(
    mut bgm_events: EventReader<PlayBgmEvent>,
    mut volume: Query<(Entity, &mut MusicVolume)>,
    audio_assets: Option<Res<AudioAssets>>,
    audio: Res<Audio>,
) {
    if audio_assets.is_none() { return; }

    for (e, mut v) in volume.iter() {
        // Update volume
        audio.set_volume(v.volume);

        // Play BGMs
        for PlayBgmEvent(bgm) in bgm_events.iter() {
            audio.stop();
            audio
                .play(bgm.get_handle(audio_assets.unwrap()))
                .looped();
            break;
        }
        bgm_events.clear();
        return;
    }
}