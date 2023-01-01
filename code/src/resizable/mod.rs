use bevy::prelude::Component;
use bevy::audio::AudioSink;
use bevy::prelude::*;

#[derive(Component)]
pub struct Resizable;

pub struct MusicController( pub Handle<AudioSink>);