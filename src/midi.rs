use midir::{MidiOutput, MidiOutputConnection};
use std::thread::sleep;
use std::time::Duration;

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;

pub struct Midi {
    midi_output: Box<MidiOutputConnection>,
}

impl Midi {
    pub fn new(connect_to: &str) -> Midi {
        let out = MidiOutput::new("dsptch").unwrap();
        let ports = out.ports();
        for port in ports.iter() {
            let port_name = out.port_name(port).unwrap();
            if port_name.starts_with(connect_to) {
                let midi_output = out.connect(port, "midi_output").unwrap();
                return Midi {
                    midi_output: Box::new(midi_output),
                };
            }
        }
        panic!("Couldn't open MIDI output")
    }

    pub fn play_note(&mut self, channel: u8, note: u8, velocity: u8, note_len: Duration) {
        // let mut midi_output = self.midi_output;
        let raw_msg = &[NOTE_ON_MSG + channel, note, velocity];
        self.midi_output.send(raw_msg).unwrap();
        sleep(note_len);
        let raw_msg = &[NOTE_OFF_MSG + channel, note, velocity];
        self.midi_output.send(raw_msg).unwrap();
    }
}
