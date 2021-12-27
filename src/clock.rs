use actix::{prelude::*, Actor, Context, Handler, Message, SyncContext};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Copy, Clone, Message)]
#[rtype(result = "()")]
pub enum ClockEvent {
    Start,
    Stop,
    SetBPM(u8),
}

pub struct Clock {
    bpm: u8,
    me: Box<Addr<Clock>>,
}

impl Clock {
    fn start_ticking(&mut self) {}

    fn stop_ticking(&mut self) {}

    pub fn new(bpm: u8) -> Clock {
        let me = Box::default();
        Clock { bpm, me }
    }
}

impl Actor for Clock {
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut SyncContext<Self>) {}
}

impl Handler<ClockEvent> for Clock {
    type Result = <ClockEvent as Message>::Result;

    fn handle(&mut self, msg: ClockEvent, _: &mut Self::Context) {
        match msg {
            ClockEvent::Start => self.start_ticking(),
            ClockEvent::Stop => self.stop_ticking(),
            ClockEvent::SetBPM(bpm) => self.bpm = bpm,
        }
    }
}
