use std::{collections::HashMap, time::Duration, sync::{Mutex, Arc}};

use rodio::{Sink, OutputStreamHandle, Source, source::{SineWave, Mix}};

use crate::{note::{Pitch, freq}, triangle_wave::TriangleWave, adsr_source::ADSRState, square_wave::SquareWave};
use crate::adsr_source::ADSRSource;

pub trait Synth {
  fn play_note(&mut self, pitch: Pitch);
  fn stop_note(&mut self, pitch: Pitch);
}

pub struct BasicSynth {
  stream_handle: OutputStreamHandle,
  playing: HashMap<Pitch, (Sink, Arc<Mutex<ADSRState>>)>,
  stopped: HashMap<Pitch, (Sink, Arc<Mutex<ADSRState>>)>,
  attack: Duration
}

impl BasicSynth {
  pub fn new(stream_handle: OutputStreamHandle) -> BasicSynth {
    BasicSynth {
      stream_handle: stream_handle,
      playing: HashMap::new(),
      stopped: HashMap::new(),
      attack: Duration::from_secs_f32(0.3)
    }
  }
}

impl Synth for BasicSynth {
    fn play_note(&mut self, pitch: Pitch) {
      if !self.playing.contains_key(&pitch) {
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        let triangle_source = TriangleWave::new(freq(pitch.clone()), 48000.0)
            .repeat_infinite()
            .amplify(0.5);
        let square_source = SquareWave::new(freq(pitch.clone()), 48000.0);
        let sine_source = SineWave::new(freq(pitch.clone()) * 4.0).amplify(0.2);
        let mix_source = triangle_source.mix(square_source).amplify(0.5);
        let state_handler = Arc::new(Mutex::new(ADSRState::attack));
        let adsred = ADSRSource::build(mix_source, state_handler.clone());
        sink.append(adsred);
        self.playing.insert(pitch.clone(), (sink, state_handler));
      }
    }

    fn stop_note(&mut self, pitch: Pitch) {
      let note = self.playing.remove(&pitch);

      match note {
        Some((sink, state_handler)) => {
          let mut state = state_handler.lock().unwrap();
          *state = ADSRState::release;
          self.stopped.insert(pitch, (sink, state_handler.clone()));
          // sink.stop();
          // sink.detach()
        },
        None => (),
      }
    }
}
