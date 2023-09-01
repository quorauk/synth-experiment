use std::f32::consts::PI;
use std::time::Duration;

use rodio::Source;


/// An infinite source that produces a sine.
///
/// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct TriangleWave {
    freq: f32,
    sample_rate: f32,
    num_sample: usize,
}

impl TriangleWave {
    /// The frequency of the sine.
    #[inline]
    pub fn new(freq: f32, sample_rate: f32) -> TriangleWave {
        TriangleWave {
            freq: freq,
            sample_rate: sample_rate,
            num_sample: 0,
        }
    }
}

impl Iterator for TriangleWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let period = self.sample_rate / self.freq;

        let floor_value = (self.num_sample as f32 / period + 0.5).floor();
        let floor_min_t_over_p = (self.num_sample as f32 / period) - floor_value;

        let value = 2.0 * (2.0 * floor_min_t_over_p).abs() - 1.0;

        Some(value)
    }
}

impl Source for TriangleWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.sample_rate as u32
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut wave = TriangleWave::new(1.0, 10.0);

    let first = wave.next();
    for i in 0..9 {
      let _ = wave.next();
    }
    let last = wave.next();

    assert!(first.unwrap() - last.unwrap() < f32::EPSILON);
  }

  #[test]
  fn it_is_never_more_than_1_or_neg_1() {
    let mut wave = TriangleWave::new(1.0, 10.0);

    for i in 0..9 {
      let x = wave.next();
      assert!(x.unwrap().abs() <= 1.0)
    }
  }
}
