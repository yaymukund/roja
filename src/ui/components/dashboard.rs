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
        helpers::write_at(self.canvas.x + 7, self.canvas.y, "/");
        self.update_current_time(0);
        self.update_total_time(0);
    }

    fn update_indicator(&self, player: &Player) {
        helpers::write_at(self.canvas.x, self.canvas.y, indicator(player));
    }

    fn update_current_time(&self, current_time: i64) {
        let current_time = format!("{:>5}", format_duration(current_time));
        helpers::write_at(self.canvas.x + 2, self.canvas.y, &current_time);
    }

    fn update_total_time(&self, total_time: i64) {
        let total_time = format!("{:<5}", format_duration(total_time));
        helpers::write_at(self.canvas.x + 8, self.canvas.y, &total_time);
    }

    fn update_progress(&self, player: &Player) {
        let cols = self.canvas.cols - 16;
        let percent_complete = player.percent_complete();
        let filled = (cols * percent_complete) / 100;

        let empty = cols - filled;
        let filled_bar = style::style("━".repeat(filled as usize)).with(style::Color::DarkMagenta);
        let empty_bar = style::style("─".repeat(empty as usize)).with(style::Color::Green);

        helpers::write_styled_at(self.canvas.x + 16, self.canvas.y, filled_bar);
        helpers::write_styled_at(self.canvas.x + 16 + filled, self.canvas.y, empty_bar);
    }

    fn disabled(&self) -> bool {
        self.canvas.cols < 26
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

fn indicator(player: &Player) -> &str {
    if player.paused() {
        "|"
    } else if player.idle_active() {
        " "
    } else {
        "▶"
    }
}
