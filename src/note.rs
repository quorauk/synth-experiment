const A440 : f32 = 440.0;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Note {
  C,
  Db,
  D,
  Eb,
  E,
  F,
  Gb,
  G,
  Ab,
  A,
  Bb,
  B,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Pitch {
  pub note: Note,
  pub octave: i32
}

fn semitones_from_c0(pitch: Pitch) -> i32 {
  let note_number = match pitch.note {
      Note::C => 0,
      Note::Db => 1,
      Note::D => 2,
      Note::Eb => 3,
      Note::E => 4,
      Note::F => 5,
      Note::Gb => 6,
      Note::G => 7,
      Note::Ab => 8,
      Note::A => 9,
      Note::Bb => 10,
      Note::B => 11,
  };

  note_number + 12 * (pitch.octave)
}

pub fn freq(pitch : Pitch) -> f32 {
  let log_temp = f32::powf(2.0, 1.0 / 12.0);
  let n: f32 = (semitones_from_c0(pitch.clone()) - semitones_from_c0(Pitch { note: Note::A, octave: 4})) as f32;
  let log_temp_for_note = f32::powf(log_temp, n);
  A440 * log_temp_for_note
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(freq(Pitch{note: Note::A, octave: 4}), 440.0);
    assert_eq!(freq(Pitch{note: Note::Bb, octave: 4}), 466.1638);
  }
}
