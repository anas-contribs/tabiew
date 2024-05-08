use std::error;

use polars::frame::DataFrame;
use ratatui::style::Style;
use tui_textarea::TextArea;

use crate::{
    theme::{Styler, Theme},
    utils::widths_from_dataframe,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct Table {
    pub data_frame: DataFrame,
    pub offset: usize,
    pub select: usize,
    pub rendered_rows: u16,
    pub widths: Vec<usize>,
    pub detailed_view: Option<Scroll>,
}

impl Table {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame) -> Self {
        Self {
            offset: 0,
            select: 0,
            rendered_rows: 0,
            widths: widths_from_dataframe(&data_frame),
            data_frame,
            detailed_view: None,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}


    pub fn select_up(&mut self, len: usize) {
        self.select(self.select.saturating_sub(len))
    }

    pub fn select_down(&mut self, len: usize) {
        self.select(self.select + len);
    }

    pub fn select_first(&mut self) {
        self.select(usize::MIN)
    }

    pub fn select_last(&mut self) {
        self.select(usize::MAX);
    }

    pub fn select(&mut self, select: usize) {
        self.select = select.min(self.data_frame.height().saturating_sub(1))
    }

    pub fn adjust_offset(&mut self) {
        self.offset = self.offset.clamp(
            self.select
                .saturating_sub(self.rendered_rows.saturating_sub(1).into()),
            self.select,
        );
    }

    pub fn switch_view(&mut self) {
        if self.detailed_view.is_none() {
            self.detailed_view = Scroll::default().into();
        } else {
            self.detailed_view = None;
        }
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) {
        self.widths = widths_from_dataframe(&data_frame);
        self.offset = 0;
        self.select = 0;
        self.data_frame = data_frame;
    }
}

#[derive(Debug, Default)]
pub enum StatusBar<'a> {
    #[default]
    Normal,
    Error(String, usize),
    Command(TextArea<'a>),
}


impl<'a> StatusBar<'a> {
    pub fn normal(&mut self) {
        self.update(StatusBar::Normal);
    }

    pub fn error(&mut self, msg: impl ToString, ticks: usize) {
        self.update(StatusBar::Error(msg.to_string(), ticks));
    }

    pub fn command(&mut self) -> &mut TextArea<'a> {
        if let StatusBar::Command(text_area) = self {
            text_area
        } else {
            let mut text_area = TextArea::default();
            text_area.set_style(Theme::status_bar_green());
            text_area.set_cursor_line_style(Style::default());
            self.update(StatusBar::Command(text_area));
            self.command()
        }
    }

    pub fn update(&mut self, status: StatusBar<'a>) {
        *self = status;
    }

    pub fn tick(&mut self) {
        if let StatusBar::Error(_, ref mut ticks) = self {
            if ticks == &0 {
                *self = StatusBar::Normal;
            } else {
                *ticks -= 1;
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Scroll(usize);

impl From<Scroll> for usize {
    fn from(val: Scroll) -> Self {
        val.0
    }
}

impl From<Scroll> for u16 {
    fn from(val: Scroll) -> Self {
        val.0 as u16
    }
}

impl Scroll {
    pub fn up(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    pub fn down(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    pub fn adjust(&mut self, lines: usize, space: usize) {
        self.0 = self.0.min(lines.saturating_sub(space))
    }
}