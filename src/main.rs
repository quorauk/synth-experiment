use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::ops::DivAssign;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, self};
use std::time::Duration;
use std::vec;
use device_query::{DeviceState, Keycode, DeviceQuery};
use rodio::buffer::SamplesBuffer;
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle, dynamic_mixer, Source};
use rodio::source::{SineWave};
use rodio::queue::{queue, SourcesQueueInput, SourcesQueueOutput};
use synth::{BasicSynth, Synth};

mod note;
mod triangle_wave;
mod square_wave;
mod synth;
mod adsr_source;

use crate::note::Pitch;
use crate::note::Note;

fn main() {
    synthed()
}

fn key_to_pitch(keycode: Keycode) -> Pitch {
    match keycode {
        Keycode::A => todo!(),
        Keycode::B => Pitch { note: Note::G, octave: 2 },
        Keycode::C => Pitch { note: Note::E, octave: 2},
        Keycode::D => Pitch { note: Note::Eb, octave: 2},
        Keycode::E => todo!(),
        Keycode::F => todo!(),
        Keycode::G => todo!(),
        Keycode::H => todo!(),
        Keycode::I => todo!(),
        Keycode::J => todo!(),
        Keycode::K => todo!(),
        Keycode::L => todo!(),
        Keycode::M => todo!(),
        Keycode::N => todo!(),
        Keycode::O => todo!(),
        Keycode::P => todo!(),
        Keycode::Q => todo!(),
        Keycode::R => todo!(),
        Keycode::S => Pitch { note: Note::Db, octave: 2},
        Keycode::T => todo!(),
        Keycode::U => todo!(),
        Keycode::V => Pitch { note: Note::F, octave: 2 },
        Keycode::W => todo!(),
        Keycode::X => Pitch { note: Note::D, octave: 2 },
        Keycode::Y => todo!(),
        Keycode::Z => Pitch { note: Note::C, octave: 2 },
        _ => todo!()
    }

}

fn synthed() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let device = DeviceState::new();
    let mut synth = BasicSynth::new(stream_handle);

    let mut playing = vec![];

    loop {
        let keys = device.get_keys();
        for key in keys {
            if !playing.contains(&key) {
                synth.play_note(key_to_pitch(key));
                playing.push(key);
            }
        }

        let keys = device.get_keys();

        let to_stop : Vec<&Keycode> = playing.iter().filter( |k| !keys.contains(k) ).collect();

        for key in to_stop {
            synth.stop_note(key_to_pitch(key.clone()))
        }

        playing.retain( |k| keys.contains(k) );
    }
} 

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
  }
}
