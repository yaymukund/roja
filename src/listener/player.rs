use crate::player::Player;
use crate::ui::{Event, IntoListener, Label, Layout, Listener, State};
use crate::util::{format_duration, Canvas, Point};

//
// Basically, we're trying to render the following:
//
// MAAAAAAAAAAAAAAAAAAAAAAAAAAAAM
// MI CCCCCC/TTTTTT PPPPPPPPPP->M
//
//  Taylor Swift - 1989
//  > -12:34/-23:45 ===--------
//
// Where:
//
// A: Artist and Song Name
// M: Margin
// I: Indicator
// C: Current Time
// T: Total Time
// P: Progress (This stretches to fill the remaining space)
//
const MARGIN_LEFT: u16 = 1;
const MARGIN_RIGHT: u16 = 1;
const OFFSET_INDICATOR: u16 = MARGIN_LEFT;
const OFFSET_INDICATOR_RMARGIN: u16 = OFFSET_INDICATOR + 1;
const OFFSET_CURRENT_TIME: u16 = OFFSET_INDICATOR + 2;
const OFFSET_SLASH: u16 = OFFSET_CURRENT_TIME + 6;
const OFFSET_TOTAL_TIME: u16 = OFFSET_SLASH + 1;
const OFFSET_TOTAL_TIME_RMARGIN: u16 = OFFSET_TOTAL_TIME + 6;
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

        self.controls()
            .right(OFFSET_INDICATOR)
            .draw(&indicator.to_string(), Label::PlayerControls);
    }

    fn draw_current_time(&self, current_time: i64) {
        let current_time = format!("{:>6}", format_duration(current_time));
        self.controls()
            .right(OFFSET_CURRENT_TIME)
            .draw(&current_time, Label::PlayerControls);
    }

    fn draw_total_time(&self, total_time: i64) {
        let total_time = format!("{:<6}", format_duration(total_time));
        self.controls()
            .right(OFFSET_TOTAL_TIME)
            .draw(&total_time, Label::PlayerControls);
    }

    fn draw_progress(&self) {
        let percent_complete = self.player.percent_complete();
        let cols = self.canvas.width() - OFFSET_PROGRESS;
        let filled = (cols * percent_complete) / 100;
        let empty = cols - filled - MARGIN_RIGHT;
        let filled_bar = "━".repeat(filled as usize);
        let empty_bar = "─".repeat(empty as usize);

        self.controls()
            .right(OFFSET_PROGRESS)
            .draw(&filled_bar, Label::PlayerProgress)
            .right(filled)
            .draw(&empty_bar, Label::PlayerProgressEmpty);
    }

    fn draw_info(&self) {
        let width = self.canvas.width();
        let now_playing = format!("{} - {}", self.player.artist(), self.player.title());
        let info = format!(
            " ♫ {:space$} ",
            now_playing,
            space = (width - MARGIN_LEFT - MARGIN_RIGHT - 2).into()
        );

        self.canvas.draw(info, Label::PlayerInfoBar);
    }

    fn draw_margins(&self) {
        let rmargin_offset = self.canvas.width() - MARGIN_RIGHT;

        for offset in &[
            0,
            OFFSET_TOTAL_TIME_RMARGIN,
            OFFSET_INDICATOR_RMARGIN,
            rmargin_offset,
        ] {
            self.controls()
                .right(*offset)
                .draw(" ", Label::PlayerControls);
        }
    }

    fn controls(&self) -> Point {
        self.canvas.down(1)
    }

    fn should_draw(&self) -> bool {
        self.canvas.width() >= 28
    }

    fn draw(&self) {
        self.draw_margins();
        self.draw_info();
        self.draw_indicator();
        self.controls()
            .right(OFFSET_SLASH)
            .draw("/", Label::PlayerControls);
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
            Event::Tick => {
                self.wait_event(ui);
            }
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
            Event::ChangeTitle => self.draw_info(),
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
