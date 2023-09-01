use std::{time::Duration, sync::{Arc, Mutex}, ops::Deref};

use rodio::{Source, Sample};

pub enum ADSRState {
  attack,
  decay,
  release,
}

pub struct ADSRSource<I> {
  inner_source: I,
  attack: Duration,
  decay: Duration,
  sustain: f32,
  release: Duration,
  state: Arc<Mutex<ADSRState>>,
  in_state_for: f32
}

impl<I> ADSRSource<I> where
  I: Source,
  I::Item: Sample {

  pub fn build(source: I, state_handler: Arc<Mutex<ADSRState>>) -> ADSRSource<I> {
    let internal_state_handler = state_handler.clone();
    ADSRSource {
      inner_source: source,
      attack: Duration::from_secs_f32(1.0),
      decay: Duration::from_secs_f32(1.0),
      sustain: 0.7,
      release: Duration::from_secs_f32(1.0),
      state: internal_state_handler,
      in_state_for: 0.0,
    }
  }
}

impl<I> Iterator for  ADSRSource<I> where
  I: Source,
  I::Item: Sample {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
      let mut change_state_to : Option<ADSRState> = None;

      let next = match self.state.lock().unwrap().deref() {
        ADSRState::attack => {
          let max_attack = self.inner_source.sample_rate() as f32 * self.attack.as_secs_f32();
          if self.in_state_for > max_attack {
            change_state_to = Some(ADSRState::decay);
            self.in_state_for = 0.0;
            self.inner_source.next()
          } else {
            self.in_state_for += 1.0;
            let value = self.in_state_for / max_attack ;
            self.inner_source.next().map(|s| s.amplify(value.clamp(0.0, 1.0)))
          }
        },
        ADSRState::decay => {
          let max_decay= self.inner_source.sample_rate() as f32 * self.decay.as_secs_f32();
          if self.in_state_for > max_decay {
            self.inner_source.next().map(|s| s.amplify(self.sustain))
          } else {
            self.in_state_for += 1.0;
            let value = self.sustain + (1.0 - self.sustain) * ((max_decay - self.in_state_for)/max_decay) ;
            self.inner_source.next().map(|s| s.amplify(value.clamp(0.0, 1.0)))
          }
        },
        ADSRState::release => {
          // weird hack
          // state management doesn't change the in state for value yet lol
          if self.in_state_for > 0.0 {
            self.in_state_for = -1.0
          } else {
            self.in_state_for -= 1.0;
          }
          let max_release= self.inner_source.sample_rate() as f32 * self.release.as_secs_f32();
          let value = self.sustain * ((max_release - (-1.0 * self.in_state_for))/max_release) ;
          self.inner_source.next().map(|s| s.amplify(value.clamp(0.0, 1.0)))
        },
      };

      match change_state_to {
        Some(x) => {
          let mut state = self.state.lock().unwrap();
          *state = x;
        },
        None => (),
      }

      return next
    }
}

impl<I> Source for ADSRSource<I> where
  I: Source,
  I::Item: Sample {
    fn current_frame_len(&self) -> Option<usize> {
      self.inner_source.current_frame_len()
    }

    fn channels(&self) -> u16 {
      self.inner_source.channels()
    }

    fn sample_rate(&self) -> u32 {
      self.inner_source.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
      self.inner_source.total_duration()
    }
}
