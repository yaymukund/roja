use crossterm::style;

use crate::player::Player;
use crate::ui::{Component, Event, IntoComponent, State};
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
const MARGIN_RIGHT: u16 = 1;
const OFFSET_INDICATOR: u16 = 0;
const OFFSET_CURRENT_TIME: u16 = OFFSET_INDICATOR + 2;
const OFFSET_SLASH: u16 = OFFSET_CURRENT_TIME + 6;
const OFFSET_TOTAL_TIME: u16 = OFFSET_SLASH + 1;
const OFFSET_PROGRESS: u16 = OFFSET_TOTAL_TIME + 7;

const INDICATOR_PAUSED: &str = "|";
const INDICATOR_IDLE: &str = " ";
const INDICATOR_PLAYING: &str = "▶";

impl IntoComponent for Player {
    type IntoComp = PlayerComponent;
    fn into_component(self, cols: u16, rows: u16) -> Self::IntoComp {
        let (point, width) = Self::IntoComp::dimensions(cols, rows);

        Self::IntoComp {
            player: self,
            point,
            width,
        }
    }
}

pub struct PlayerComponent {
    player: Player,
    point: Point,
    width: u16,
}

impl PlayerComponent {
    fn dimensions(cols: u16, rows: u16) -> (Point, u16) {
        if rows == 0 || cols < 2 {
            panic!("dimensions for player too small");
        }

        (point!(1, rows - 1), cols - 2)
    }

    fn draw_indicator(&self) {
        let indicator = if self.player.paused() {
            INDICATOR_PAUSED
        } else if self.player.idle_active() {
            INDICATOR_IDLE
        } else {
            INDICATOR_PLAYING
        };

        self.point.right(OFFSET_INDICATOR).write(indicator);
    }

    fn draw_current_time(&self, current_time: i64) {
        let current_time = format!("{:>6}", format_duration(current_time));
        self.point.right(OFFSET_CURRENT_TIME).write(&current_time);
    }

    fn draw_total_time(&self, total_time: i64) {
        let total_time = format!("{:<6}", format_duration(total_time));
        self.point.right(OFFSET_TOTAL_TIME).write(&total_time);
    }

    fn draw_progress(&self) {
        let percent_complete = self.player.percent_complete();
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

impl Component for PlayerComponent {
    fn draw(&self) {
        if !self.should_render() {
            return;
        }

        self.draw_indicator();
        self.point.right(OFFSET_SLASH).write("/");
        self.draw_current_time(self.player.elapsed());
        self.draw_total_time(self.player.duration());
        self.draw_progress();
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        let (point, width) = Self::dimensions(cols, rows);
        self.point = point;
        self.width = width;
    }

    fn on_tick(&self, ui: &mut State) {
        if let Some(event) = self.player.wait_event() {
            ui.dispatch(event.into());
        }
    }

    fn on_event(&mut self, event: &Event, _ui: &mut State) {
        match *event {
            Event::SeekForward => self.player.seek_forward(),
            Event::SeekBackward => self.player.seek_backward(),
            Event::TogglePause => self.player.toggle_pause(),
            _ => {}
        }

        if !self.should_render() {
            return;
        }

        match *event {
            Event::SeekForward | Event::SeekBackward => {
                self.draw_current_time(self.player.elapsed())
            }
            Event::ChangeIndicator | Event::TogglePause => self.draw_indicator(),
            Event::ChangeCurrentTime(secs) => {
                self.draw_current_time(secs);
                self.draw_progress();
            }
            Event::ChangeTotalTime(secs) => {
                self.draw_total_time(secs);
                self.draw_progress();
            }
            _ => {}
        }
    }
}
