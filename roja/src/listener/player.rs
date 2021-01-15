use crate::player::{Player, SeekableRanges};
use crate::store::Playlist;
use crate::ui::{layout, Direction, Event, IntoListener, Label, Listener};
use crate::util::{channel, fit_width, format_duration, Canvas, Point};
use anyhow::Result;

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

const INDICATOR_PAUSED: &str = "|";
const INDICATOR_IDLE: &str = " ";
const INDICATOR_PLAYING: &str = "▶";
const PROGRESS_FILLED: &str = "━";
const PROGRESS_UNPLAYED: &str = "─";

impl IntoListener for Player<'static> {
    type LType = PlayerComponent;
    fn into_listener(self, _sender: channel::Sender<Event>) -> Self::LType {
        Self::LType {
            active: true,
            player: self,
            canvas: Canvas::Uninitialized,
            seekable_ranges: Vec::new(),
        }
    }
}

pub struct PlayerComponent {
    active: bool,
    player: Player<'static>,
    canvas: Canvas,
    seekable_ranges: SeekableRanges,
}

impl PlayerComponent {
    fn draw_indicator(&self) {
        let indicator = if self.player.paused() {
            INDICATOR_PAUSED
        } else if self.player.is_track_loaded() {
            INDICATOR_PLAYING
        } else {
            INDICATOR_IDLE
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
        let elapsed = self.player.elapsed() as f64;
        let duration = self.player.duration() as f64;
        let rem_cols = self.canvas.width() - OFFSET_PROGRESS - MARGIN_RIGHT;

        let mut progress_bar = ProgressBar {
            sec_cols: (rem_cols as f64) / duration,
            rem_cols,
            point: self.controls().right(OFFSET_PROGRESS),
        };

        // fill elapsed
        progress_bar.fill(elapsed, PROGRESS_FILLED, Label::PlayerProgress);

        let mut curr = elapsed;
        for (mut start, mut end) in &self.seekable_ranges {
            if end <= curr {
                continue;
            }

            if start < curr {
                start = curr;
            }

            if end > duration {
                end = duration;
            }

            // fill unplayed segment from cursor to start
            if start > curr {
                progress_bar.fill(start - curr, PROGRESS_UNPLAYED, Label::PlayerProgressEmpty);
            }

            // fill buffered segment from start to end
            progress_bar.fill(
                end - start,
                PROGRESS_UNPLAYED,
                Label::PlayerProgressBuffered,
            );

            // move cursor
            curr = end;
        }

        progress_bar.fill_remaining(PROGRESS_UNPLAYED, Label::PlayerProgressEmpty);
    }

    fn draw_info(&self) {
        let info = format!(" ♫ {} - {}", self.player.artist(), self.player.title());
        let info = fit_width(&info, self.canvas.width() as usize, false);

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
        self.canvas.is_initialized() && self.canvas.width() >= 28
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

    fn resize(&mut self, width: u16, height: u16) {
        self.canvas = layout::player_canvas(width, height);
    }

    fn queue_tracks(&self, playlist: &Playlist) {
        self.player.stop();
        for track in playlist.tracks.iter() {
            self.player.playlist_append(&track.path);
        }
        self.player.playlist_play_index(playlist.selected_index);
    }

    fn seek(&self, direction: Direction) {
        if !self.player.is_track_loaded() || !self.active {
            return;
        }

        match direction {
            Direction::Left => self.player.seek_backward(),
            Direction::Right => self.player.seek_forward(),
            _ => {}
        }
    }

    fn toggle_pause(&self) {
        if self.active && self.player.is_track_loaded() {
            self.player.toggle_pause();
        }
    }

    fn update_seekable_ranges(&mut self, new_ranges: SeekableRanges) {
        self.seekable_ranges = new_ranges;
        self.draw_progress();
    }
}

impl Listener for PlayerComponent {
    fn on_event(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::FocusSearch => self.active = false,
            Event::CancelSearch => self.active = true,
            Event::QueuePlaylist(playlist) => self.queue_tracks(playlist),
            Event::Resize(width, height) => self.resize(*width, *height),
            _ => {
                if let Some('c') = event.pressed_char() {
                    self.toggle_pause();
                } else if let Some(dir) = event.key_event_direction() {
                    self.seek(dir);
                }
            }
        }

        if !self.should_draw() {
            return Ok(());
        }

        match event {
            Event::Draw => self.draw(),
            Event::ChangeTitle => self.draw_info(),
            Event::ChangeIdle | Event::ChangeIndicator => self.draw_indicator(),
            Event::ChangeCurrentTime(secs) => {
                self.draw_current_time(*secs);
                self.draw_progress();
            }
            Event::ChangeTotalTime(secs) => {
                self.draw_total_time(*secs);
                self.draw_progress();
            }
            Event::ChangeSeekableRanges(ranges) => self.update_seekable_ranges(ranges.clone()),
            _ => {
                if let Some('c') = event.pressed_char() {
                    self.draw_indicator()
                } else if let Some(dir) = event.key_event_direction() {
                    if dir == Direction::Left || dir == Direction::Right {
                        self.draw_current_time(self.player.elapsed())
                    }
                }
            }
        }

        Ok(())
    }
}

struct ProgressBar {
    sec_cols: f64,
    rem_cols: u16,
    point: Point,
}

impl ProgressBar {
    fn fill(&mut self, secs: f64, character: &str, label: Label) {
        if self.rem_cols == 0 {
            return;
        }

        let mut cols = (secs * self.sec_cols).ceil() as u16;

        if cols > self.rem_cols {
            cols = self.rem_cols;
        }

        self.rem_cols -= cols;

        let fill = character.repeat(cols as usize);
        self.point = self.point.draw(&fill, label).right(cols);
    }

    fn fill_remaining(self, character: &str, label: Label) {
        if self.rem_cols == 0 {
            return;
        }

        let fill = character.repeat(self.rem_cols as usize);
        self.point.draw(&fill, label);
    }
}
