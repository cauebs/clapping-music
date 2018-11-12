use midir::{MidiOutput, MidiOutputConnection};
use pitch_calc::{Letter, LetterOctave, Octave};
use rimd::MidiMessage;

use std::{error::Error, thread::sleep, time::Duration};

struct Piano {
    connection: MidiOutputConnection,
}

impl Piano {
    fn from_port(number: usize) -> Result<Self, Box<Error>> {
        let output = MidiOutput::new("piano_client")?;
        let connection = output.connect(number, "piano")?;
        Ok(Self { connection })
    }

    fn play(
        &mut self,
        notes: &[(Letter, Octave)],
        velocity: u8,
        duration: u64,
    ) -> Result<(), Box<Error>> {
        if notes.is_empty() {
            sleep(Duration::from_millis(duration));
        } else {
            self.press(notes, velocity)?;
            sleep(Duration::from_millis(duration));
            self.release(notes, velocity)?;
        }

        Ok(())
    }

    fn press(&mut self, notes: &[(Letter, Octave)], velocity: u8) -> Result<(), Box<Error>> {
        for (letter, octave) in notes {
            let note = LetterOctave(*letter, *octave).step() as u8;

            let message = MidiMessage::note_on(note, velocity, 0);
            self.connection.send(&message.data)?;
        }

        Ok(())
    }

    fn release(&mut self, notes: &[(Letter, Octave)], velocity: u8) -> Result<(), Box<Error>> {
        for (letter, octave) in notes {
            let note = LetterOctave(*letter, *octave).step() as u8;

            let message = MidiMessage::note_off(note, velocity, 0);
            self.connection.send(&message.data)?;
        }

        Ok(())
    }
}

fn print_marks(marks: &[bool], offset: usize, step: usize) {
    print!("{}", " ".repeat(10 + offset * 3));

    let n = marks.len() as isize;
    let step = ((((step as isize - offset as isize) % n) + n) % n) as usize;

    for (i, mark) in marks.iter().enumerate() {
        let block = if !mark {
            " "
        } else if i != step {
            "░"
        } else {
            "█"
        };
        print!("{} ", block.repeat(2));
    }

    println!();
}

fn play_voices(
    piano: &mut Piano,
    velocity: u8,
    marks: &[bool],
    iteration: usize,
    last_time: bool,
) -> Result<(), Box<Error>> {
    let voice1 = marks;
    let mut voice2 = marks.to_owned();
    voice2.rotate_right(iteration);

    let merged_voice = voice1.iter().zip(voice2.iter());

    for (i, (&mark1, &mark2)) in merged_voice.enumerate() {
        print_marks(marks, 0, i);
        println!();
        print_marks(marks, iteration, i);
        print!("{}", "\n".repeat(20));

        let mut notes = Vec::new();

        if mark1 {
            notes.push((Letter::C, 3));
            notes.push((Letter::D, 4));
            notes.push((Letter::E, 5));
        }

        if mark2 {
            notes.push((Letter::G, 3));
            notes.push((Letter::A, 4));
            notes.push((Letter::B, 5));
        }

        if !last_time || i < voice1.len() - 2 {
            piano.play(&notes, velocity, 200)?;
        } else {
            piano.play(&notes, velocity + 10, 1000)?;
        }
    }

    Ok(())
}

fn clapping_music(piano: &mut Piano) -> Result<(), Box<Error>> {
    let marks = [
        true, true, true, false, true, true, false, true, false, true, true, false,
    ];

    for i in 0..marks.len() {
        for _ in 0..4 {
            play_voices(piano, 40, &marks, i, false)?;
        }
    }

    for _ in 0..3 {
        play_voices(piano, 50, &marks, 0, false)?;
    }
    play_voices(piano, 60, &marks, 0, true)?;

    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    print!("{}", "\n".repeat(20));
    let mut piano = Piano::from_port(1)?;
    clapping_music(&mut piano)?;
    Ok(())
}
