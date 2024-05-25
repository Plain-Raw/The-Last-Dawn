use crate::plugins::init::setup::MenuMusicVolume;
use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, AudioSink, AudioSinkPlayback, PlaybackMode, PlaybackSettings};
use bevy::log::info;
use bevy::prelude::{default, Commands, Component, DetectChanges, Query, Res, With};
use std::ops::Div;

pub fn setup_menu_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/piano-loop.ogg"),
            settings: {
                PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    speed: 1.0,
                    ..default()
                }
            },
        },
        MenuMusic,
    ));
}

#[derive(Component)]
pub struct MenuMusic;

pub fn pause_menu_music(music_controller: Query<&AudioSink, With<MenuMusic>>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.stop()
    }
}

pub fn on_menu_music_volume_change(
    my_res: Option<Res<MenuMusicVolume>>,
    music_controller: Query<&AudioSink, With<MenuMusic>>,
) {
    if let Some(my_res) = my_res {
        // the resource exists
        if my_res.is_changed() {
            if let Ok(sink) = music_controller.get_single() {
                let f = my_res.as_f32();
                info!("volume changed:{}", f);
                if f == 0. {
                    sink.set_volume(0.);
                } else {
                    sink.set_volume(f.div(10.));
                }
            }
        }
    }
}
