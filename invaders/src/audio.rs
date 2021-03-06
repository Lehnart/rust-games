use sdl2::mixer::{Channel, Chunk};

use engine::audio::init_audio;

use crate::logic::Logic;

pub const CHANNEL_COUNT: i32 = 6;

pub struct Audio {}

impl Audio {
    pub fn new() -> Audio {
        init_audio(CHANNEL_COUNT);

        Audio {}
    }

    pub fn update(&mut self, _logic: &Logic) {}
}
