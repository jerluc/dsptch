use actix::Addr;
use druid::widget::{Align, Button, Flex, Label};
use druid::{AppLauncher, Data, Lens, WindowDesc};

use std::sync::Arc;

use crate::clock::{Clock, ClockEvent};

#[derive(Clone, Data, Lens)]
struct AppState {
    clock: Arc<Addr<Clock>>,
    playing: bool,
}

pub struct MainUI {
    launcher: AppLauncher<AppState>,
}

impl MainUI {
    pub fn new() -> MainUI {
        let main_window = WindowDesc::new(move || {
            let label: Label<AppState> = Label::dynamic(|state: &AppState, _env| {
                if state.playing {
                    format!("Stop")
                } else {
                    format!("Start")
                }
            });
            let btn = Button::from_label(label).on_click(|_ctx, state: &mut AppState, _env| {
                if state.playing {
                    state.clock.try_send(ClockEvent::Stop).unwrap();
                } else {
                    state.clock.try_send(ClockEvent::Start).unwrap();
                }
                state.playing = !state.playing;
            });

            // arrange the two widgets vertically, with some padding
            let layout = Flex::column().with_child(btn);

            // center the two widgets in the available space
            Align::centered(layout)
        })
        .title("dsptch")
        .window_size((400.0, 400.0));

        // start the application
        let launcher = AppLauncher::with_window(main_window);

        MainUI { launcher }
    }

    pub fn start(self, clock: Arc<Addr<Clock>>) {
        let initial_state = AppState {
            clock,
            playing: false,
        };
        self.launcher
            .launch(initial_state)
            .expect("Failed to launch application");
    }
}
