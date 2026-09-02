pub mod resampler;
pub mod player;

use gba_emulator::gba::GBA;
use log::error;
use player::AudioPlayer;
use std::cell::RefCell;
use std::rc::Rc;

pub fn push_audio_samples(gba: &Rc<RefCell<GBA>>, audio_player: &Rc<RefCell<Option<AudioPlayer>>>, recent_samples: &Rc<RefCell<Vec<i16>>>) {
    let samples = std::mem::take(&mut gba.borrow_mut().apu.sample_buffer);
    *recent_samples.borrow_mut() = samples.clone();
    if let Some(player) = audio_player.borrow_mut().as_mut() {
        if let Err(e) = player.push_samples(&samples) {
            error!("Failed to push audio samples: {:?}", e);
        }
    }
}
