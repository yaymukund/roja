use crossterm::style;

use crate::player::Player;
use crate::ui::{Event, Listener, State};
use crate::util::{format_duration, Point};

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
    point: Point,
    width: u16,
}

impl Renderer {
    fn draw(&self) {
        self.draw_indicator(INDICATOR_IDLE);
        self.point.right(OFFSET_SLASH).write("/");
        self.draw_current_time(0);
        self.draw_total_time(0);
    }
    fn draw_indicator(&self, indicator: &str) {
        self.point.right(MARGIN_LEFT).write(indicator);
    }

    fn draw_current_time(&self, current_time: i64) {
        let current_time = format!("{:>6}", format_duration(current_time));
        self.point.right(OFFSET_CURRENT_TIME).write(&current_time);
    }

    fn draw_total_time(&self, total_time: i64) {
        let total_time = format!("{:<6}", format_duration(total_time));
        self.point.right(OFFSET_TOTAL_TIME).write(&total_time);
    }

    fn draw_progress(&self, percent_complete: u16) {
        let cols = self.width - OFFSET_PROGRESS - MARGIN_RIGHT;
        let filled = (cols * percent_complete) / 100;
        let empty = cols - filled;

        let filled_bar = style::style("━".repeat(filled as usize)).with(style::Color::DarkMagenta);
        let empty_bar = style::style("─".repeat(empty as usize)).with(style::Color::Green);

        self.point
            .right(OFFSET_PROGRESS)
            .write_styled(filled_bar)
            .right(filled)
            .write_styled(empty_bar);
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
    fn on_event(&mut self, event: &Event, state: &mut State) {
        let renderer = Renderer {
            point: point!(0, state.rows() - 1),
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
