mod clock;
mod midi;
mod ui;

use actix::{Actor, Addr, Context, Handler, Message, SyncArbiter, SyncContext, System};
use std::io;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::clock::Clock;
use crate::midi::Midi;
use crate::ui::MainUI;

#[derive(Debug, Copy, Clone, Message)]
#[rtype(result = "()")]
pub enum MidiEvent {
    NoteOn(Instant, u8, u8, u8),
    NoteOff(Instant, u8, u8, u8),
}

pub struct MidiActor {
    midi: Midi,
}

impl Actor for MidiActor {
    type Context = SyncContext<Self>;
}

impl Handler<MidiEvent> for MidiActor {
    type Result = <MidiEvent as Message>::Result;
    fn handle(&mut self, msg: MidiEvent, _: &mut Self::Context) {
        println!("Received MIDI message: {:?}", msg);
        match msg {
            MidiEvent::NoteOn(t, channel, note, velocity) => {
                let dur = Instant::now().saturating_duration_since(t);
                println!("[{} ms]", dur.as_millis());
                self.midi
                    .play_note(channel, note, velocity, Duration::from_millis(200));
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Message)]
#[rtype(result = "()")]
pub enum TimelineEvent {
    Play,
    Pause,
    Stop,
    Reset,
}

pub struct TimelineActor {
    output: Box<Addr<MidiActor>>,
}

impl Actor for TimelineActor {
    type Context = Context<Self>;
}

impl Handler<TimelineEvent> for TimelineActor {
    type Result = <TimelineEvent as Message>::Result;
    fn handle(&mut self, msg: TimelineEvent, _ctx: &mut Self::Context) {
        match msg {
            TimelineEvent::Play => {
                let t = Instant::now();
                self.output.do_send(MidiEvent::NoteOn(t, 0, 60, 100));
                thread::sleep(Duration::from_millis(250));
            }
            TimelineEvent::Stop => {}
            _ => todo!(),
        }
    }
}

fn main() -> io::Result<()> {
    let system = System::new();

    let clock = SyncArbiter::start(1, move || Clock::new(120));

    thread::spawn(move || {
        let ui = MainUI::new();
        ui.start(Arc::new(clock.clone()));
    });

    system.run()
}
