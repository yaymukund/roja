use crossterm::style::style;

use crate::player::Player;
use crate::ui::{Event, IntoListener, Layout, Listener, State};
use crate::util::{format_duration, Canvas};
use crate::Settings;

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

const INDICATOR_PAUSED: char = '|';
const INDICATOR_IDLE: char = ' ';
const INDICATOR_PLAYING: char = '▶';

impl IntoListener for Player {
    type LType = PlayerComponent;
    fn into_listener(self, layout: &Layout) -> Self::LType {
        Self::LType {
            player: self,
            canvas: layout.player.clone(),
        }
    }
}

pub struct PlayerComponent {
    player: Player,
    canvas: Canvas,
}

impl PlayerComponent {
    fn draw_indicator(&self) {
        let indicator = if self.player.paused() {
            INDICATOR_PAUSED
        } else if self.player.idle_active() {
            INDICATOR_IDLE
        } else {
            INDICATOR_PLAYING
        };

        self.canvas
            .right(OFFSET_INDICATOR)
            .write(&indicator.to_string());
    }

    fn draw_current_time(&self, current_time: i64) {
        let current_time = format!("{:>6}", format_duration(current_time));
        self.canvas.right(OFFSET_CURRENT_TIME).write(&current_time);
    }

    fn draw_total_time(&self, total_time: i64) {
        let total_time = format!("{:<6}", format_duration(total_time));
        self.canvas.right(OFFSET_TOTAL_TIME).write(&total_time);
    }

    fn draw_progress(&self) {
        let percent_complete = self.player.percent_complete();
        let cols = self.canvas.width() - OFFSET_PROGRESS - MARGIN_RIGHT;
        let filled = (cols * percent_complete) / 100;
        let empty = cols - filled;

        let filled_bar = style("━".repeat(filled as usize))
            .with(*Settings::global().colors().progress_bar_fill());
        let empty_bar = style("─".repeat(empty as usize))
            .with(*Settings::global().colors().progress_bar_empty());

        self.canvas
            .right(OFFSET_PROGRESS)
            .write_styled(filled_bar)
            .right(filled)
            .write_styled(empty_bar);
    }

    fn should_draw(&self) -> bool {
        self.canvas.width() >= 28
    }

    fn draw(&self) {
        self.draw_indicator();
        self.canvas.right(OFFSET_SLASH).write("/");
        self.draw_current_time(self.player.elapsed());
        self.draw_total_time(self.player.duration());
        self.draw_progress();
    }

    fn wait_event(&self, ui: &mut State) {
        if let Some(event) = self.player.wait_event() {
            ui.dispatch(event.into());
        }
    }

    fn resize(&mut self, layout: &Layout) {
        self.canvas = layout.player.clone();
    }
}

impl Listener for PlayerComponent {
    fn on_event(&mut self, event: &Event, ui: &mut State) {
        match event {
            Event::ResizeListener(layout) => self.resize(layout),
            Event::Tick => self.wait_event(ui),
            Event::SeekForward => self.player.seek_forward(),
            Event::SeekBackward => self.player.seek_backward(),
            Event::TogglePause => self.player.toggle_pause(),
            _ => {}
        }

        if !self.should_draw() {
            return;
        }

        match event {
            Event::Draw => self.draw(),
            Event::SeekForward | Event::SeekBackward => {
                self.draw_current_time(self.player.elapsed())
            }
            Event::ChangeIndicator | Event::TogglePause => self.draw_indicator(),
            Event::ChangeCurrentTime(secs) => {
                self.draw_current_time(*secs);
                self.draw_progress();
            }
            Event::ChangeTotalTime(secs) => {
                self.draw_total_time(*secs);
                self.draw_progress();
            }
            _ => {}
        }
    }
}
