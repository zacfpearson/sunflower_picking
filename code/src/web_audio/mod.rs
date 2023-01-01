use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use bevy::audio::AudioSink;
use bevy::ecs::schedule::SystemSet;
use bevy::prelude::*;
use bevy::window::{WindowResizeConstraints};
use crate::resizable::MusicController;

#[wasm_bindgen]
extern "C" {
    pub fn get_audio_toggled() -> bool;

    pub fn set_audio_toggled(state: bool);
}

#[cfg(target_arch = "wasm32")]
pub fn web_audio_control(
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
)  {
    if get_audio_toggled() {
        if let Some(sink) = audio_sinks.get(&music_controller.0) {
            if sink.is_paused() {
                sink.play()
            } else {
                sink.pause()
            }
            set_audio_toggled(false);
        }
    }
}