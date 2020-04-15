use crossterm::style;

use crate::player::Player;
use crate::runtime::RcRuntime;
use crate::ui::{helpers, Canvas, UIComponent, UIEvent};
use crate::util::format_duration;

pub(crate) struct Dashboard {
    canvas: Canvas,
}

// The offsets are a bit weird but basically, we're trying to render the
// following:
//
// I CCCCCC/TTTTTT PPPPPPPPP->
// > -12:34/-23:45 ===--------
//
// Where:
//
// I: Indicator
// C: Current Time
// T: Total Time
// P: Progress (This stretches to fill the remaining space)
impl Dashboard {
    pub fn new(canvas: Canvas) -> Dashboard {
        Dashboard { canvas }
    }

    fn draw(&self, player: &Player) {
        self.update_indicator(player);
        helpers::write_at(self.canvas.x1 + 8, self.canvas.y1, "/");
        self.update_current_time(0);
        self.update_total_time(0);
    }

    fn update_indicator(&self, player: &Player) {
        let indicator = if player.paused() {
            "|"
        } else if player.idle_active() {
            " "
        } else {
            "▶"
        };
        helpers::write_at(self.canvas.x1, self.canvas.y1, indicator);
    }

    fn update_current_time(&self, current_time: i64) {
        let current_time = format!("{:>6}", format_duration(current_time));
        helpers::write_at(self.canvas.x1 + 2, self.canvas.y1, &current_time);
    }

    fn update_total_time(&self, total_time: i64) {
        let total_time = format!("{:<6}", format_duration(total_time));
        helpers::write_at(self.canvas.x1 + 9, self.canvas.y1, &total_time);
    }

    fn update_progress(&self, player: &Player) {
        let cols = self.canvas.width() - 16;
        let percent_complete = player.percent_complete();
        let filled = (cols * percent_complete) / 100;

        let empty = cols - filled;
        let filled_bar = style::style("━".repeat(filled as usize)).with(style::Color::DarkMagenta);
        let empty_bar = style::style("─".repeat(empty as usize)).with(style::Color::Green);

        helpers::write_styled_at(self.canvas.x1 + 16, self.canvas.y1, filled_bar);
        helpers::write_styled_at(self.canvas.x1 + 16 + filled, self.canvas.y1, empty_bar);
    }

    fn disabled(&self) -> bool {
        self.canvas.width() < 26
    }
}

impl UIComponent for Dashboard {
    fn on_event(&self, event: &UIEvent, runtime: RcRuntime) {
        if self.disabled() {
            return;
        }

        let player = &runtime.borrow().player;
        match *event {
            UIEvent::Draw => self.draw(player),
            UIEvent::ChangeIndicator => self.update_indicator(player),
            UIEvent::ChangeCurrentTime(secs) => {
                self.update_current_time(secs);
                self.update_progress(player);
            }
            UIEvent::ChangeTotalTime(secs) => {
                self.update_total_time(secs);
                self.update_progress(player);
            }
            _ => {}
        }
    }

    fn after_event(&self, event: &UIEvent, runtime: RcRuntime) {
        if self.disabled() {
            return;
        }

        let player = &runtime.borrow().player;
        match *event {
            UIEvent::SeekBackward | UIEvent::SeekForward => {
                self.update_current_time(player.elapsed());
                self.update_progress(player);
            }

            UIEvent::TogglePause => self.update_indicator(player),
            _ => {}
        }
    }
}
