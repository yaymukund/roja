use crossterm::style;

use crate::player::Player;
use crate::ui::{Event, Listener, State};
use crate::util::{format_duration, terminal};

//
// Basically, we're trying to render the following:
//
// MI CCCCCC/TTTTTT PPPPPPPPP->M
//  > -12:34/-23:45 ===--------
//
// Where:
//
// M: Margin
// I: Indicator
// C: Current Time
// T: Total Time
// P: Progress (This stretches to fill the remaining space)
//
const MARGIN_LEFT: u16 = 1;
const MARGIN_RIGHT: u16 = 1;
const OFFSET_INDICATOR: u16 = MARGIN_LEFT;
const OFFSET_CURRENT_TIME: u16 = OFFSET_INDICATOR + 2;
const OFFSET_SLASH: u16 = OFFSET_CURRENT_TIME + 6;
const OFFSET_TOTAL_TIME: u16 = OFFSET_SLASH + 1;
const OFFSET_PROGRESS: u16 = OFFSET_TOTAL_TIME + 7;

const INDICATOR_PAUSED: &str = "|";
const INDICATOR_IDLE: &str = " ";
const INDICATOR_PLAYING: &str = "▶";

struct Renderer {
    y: u16,
    width: u16,
}

impl Renderer {
    fn draw(&self) {
        self.draw_indicator(INDICATOR_IDLE);
        self.write(OFFSET_SLASH, "/");
        self.draw_current_time(0);
        self.draw_total_time(0);
    }
    fn draw_indicator(&self, indicator: &str) {
        self.write(MARGIN_LEFT, indicator);
    }

    fn draw_current_time(&self, current_time: i64) {
        let current_time = format!("{:>6}", format_duration(current_time));
        self.write(OFFSET_CURRENT_TIME, &current_time);
    }

    fn draw_total_time(&self, total_time: i64) {
        let total_time = format!("{:<6}", format_duration(total_time));
        self.write(OFFSET_TOTAL_TIME, &total_time);
    }

    fn draw_progress(&self, percent_complete: u16) {
        let cols = self.width - OFFSET_PROGRESS - MARGIN_RIGHT;
        let filled = (cols * percent_complete) / 100;

        let empty = cols - filled;
        let filled_bar = style::style("━".repeat(filled as usize)).with(style::Color::DarkMagenta);
        let empty_bar = style::style("─".repeat(empty as usize)).with(style::Color::Green);

        terminal::write_styled_at(OFFSET_PROGRESS, self.y, filled_bar);
        terminal::write_styled_at(OFFSET_PROGRESS + filled, self.y, empty_bar);
    }

    fn write(&self, x: u16, text: &str) {
        terminal::write_at(x, self.y, text);
    }

    fn should_render(&self) -> bool {
        self.width >= 28
    }
}

impl Player {
    fn indicator(&self) -> &str {
        if self.paused() {
            INDICATOR_PAUSED
        } else if self.idle_active() {
            INDICATOR_IDLE
        } else {
            INDICATOR_PLAYING
        }
    }
}

impl Listener for Player {
    fn on_event(&self, event: &Event, state: &mut State) {
        let renderer = Renderer {
            y: state.rows() - 1,
            width: state.cols(),
        };

        match *event {
            Event::SeekForward => self.seek_forward(),
            Event::SeekBackward => self.seek_backward(),
            Event::TogglePause => self.toggle_pause(),
            _ => {}
        }

        if !renderer.should_render() {
            return;
        }

        match *event {
            Event::Draw => renderer.draw(),
            Event::SeekForward | Event::SeekBackward => renderer.draw_current_time(self.elapsed()),
            Event::ChangeIndicator | Event::TogglePause => {
                renderer.draw_indicator(self.indicator())
            }
            Event::ChangeCurrentTime(secs) => {
                renderer.draw_current_time(secs);
                renderer.draw_progress(self.percent_complete());
            }
            Event::ChangeTotalTime(secs) => {
                renderer.draw_total_time(secs);
                renderer.draw_progress(self.percent_complete());
            }
            _ => {}
        }
    }

    fn wait_event(&self) -> Option<Event> {
        if let Some(event) = self.wait_event() {
            Some(event.into())
        } else {
            None
        }
    }
}
