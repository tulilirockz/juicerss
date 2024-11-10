use crate::config::ListFormat;
use crate::{Config, FeedWithCustom};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, List, ListItem, ListState, Paragraph, StatefulWidget, Widget, Wrap},
    DefaultTerminal,
};
use std::io::Write;
use std::process::{Command, Stdio};
use std::{fmt::Debug, io::Cursor};

#[derive(Debug)]
struct AppTheme {
    accent: Style,
    text: Style,
    error: Style,
}

#[derive(Debug, Clone)]
pub struct ScrollUnitHorizontal {
    pub value: u16,
    factor: u16,
}
#[derive(Debug, Clone)]
pub struct ScrollUnitVertical {
    pub value: u16,
    factor: u16,
}

pub enum ScrollDirectionVertical {
    Up,
    Down,
}
pub enum ScrollDirectionHorizontal {
    Left,
    Right,
}

impl ScrollableUnit<ScrollDirectionVertical> for ScrollUnitVertical {
    fn scroll(&mut self, value: u16, direction: ScrollDirectionVertical) -> Result<(), String> {
        match direction {
            ScrollDirectionVertical::Up => {
                self.value = self
                    .value
                    .checked_sub(value)
                    .ok_or("Failed scrolling, whatever")?
                    * self.factor;
                Ok(())
            }
            ScrollDirectionVertical::Down => {
                self.value = self
                    .value
                    .checked_add(value)
                    .ok_or("Failed scrolling, whatever")?
                    * self.factor;
                Ok(())
            }
        }
    }
    fn reset(&mut self) {
        self.value = 0;
    }
}

impl ScrollableUnit<ScrollDirectionHorizontal> for ScrollUnitHorizontal {
    fn scroll(&mut self, value: u16, direction: ScrollDirectionHorizontal) -> Result<(), String> {
        match direction {
            ScrollDirectionHorizontal::Left => {
                self.value = self
                    .value
                    .checked_sub(value)
                    .ok_or("Failed scrolling, whatever")?
                    * self.factor;
                Ok(())
            }
            ScrollDirectionHorizontal::Right => {
                self.value = self
                    .value
                    .checked_add(value)
                    .ok_or("Failed scrolling, whatever")?
                    * self.factor;
                Ok(())
            }
        }
    }
    fn reset(&mut self) {
        self.value = 0;
    }
}

impl Default for ScrollUnitVertical {
    fn default() -> Self {
        Self {
            value: 0,
            factor: 1,
        }
    }
}
impl Default for ScrollUnitHorizontal {
    fn default() -> Self {
        Self {
            value: 0,
            factor: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScrollState(ScrollUnitVertical, ScrollUnitHorizontal);

pub trait ScrollableUnit<Direction> {
    fn scroll(&mut self, value: u16, direction: Direction) -> Result<(), String>;
    fn reset(&mut self);
}

impl From<ScrollState> for (u16, u16) {
    fn from(value: ScrollState) -> Self {
        (value.0.value, value.1.value)
    }
}

impl ScrollState {
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
    }
}

#[derive(Debug)]
pub struct App {
    config: Config,
    theme: AppTheme,
    screen: CurrentScreen,
    selected_feed_idx: usize,
    feeds: Vec<Option<FeedWithCustom>>,
    exit: bool,
    scroll_number: ScrollState,
    list_state: ListState,
    selected_entry: Option<feed_rs::model::Entry>,
    buffered_render: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentScreen {
    Selection,
    SingleArticle,
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self.screen {
            CurrentScreen::Selection => self.render_list(area, buf),
            CurrentScreen::SingleArticle => self.render_article(area, buf),
        }
    }
}

impl App {
    const LARGE_NUMBER: usize = 5000;

    pub fn new(feeds: Vec<Option<FeedWithCustom>>, config: Config) -> Self {
        Self {
            feeds,
            exit: false,
            screen: CurrentScreen::Selection,
            selected_feed_idx: 0,
            list_state: ListState::default(),
            selected_entry: None,
            scroll_number: ScrollState(
                ScrollUnitVertical {
                    value: config.scrolling.y_lines,
                    factor: config.scrolling.y_factor,
                },
                ScrollUnitHorizontal {
                    value: config.scrolling.x_lines,
                    factor: config.scrolling.x_factor,
                },
            ),
            buffered_render: None,
            theme: AppTheme {
                error: Style::new().fg(ratatui::style::Color::Rgb(
                    config.theme.error.red,
                    config.theme.error.green,
                    config.theme.error.blue,
                )),
                accent: Style::new().fg(ratatui::style::Color::Rgb(
                    config.theme.accent.red,
                    config.theme.accent.green,
                    config.theme.accent.blue,
                )),
                text: Style::new().fg(ratatui::style::Color::Rgb(
                    config.theme.text.red,
                    config.theme.text.green,
                    config.theme.text.blue,
                )),
            },
            config,
        }
    }

    fn render_article(&self, area: Rect, buf: &mut Buffer) {
        let init_block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title_position(ratatui::widgets::block::Position::Top)
            .title_alignment(ratatui::layout::Alignment::Center)
            .style(self.theme.accent)
            .title_top(
                Line::from(format!(
                    "{} Esc",
                    if self.config.nerd_fonts { "" } else { "<" }
                ))
                .left_aligned(),
            )
            .title(
                Line::from(self.selected_entry.clone().unwrap().title.unwrap().content)
                    .style(self.theme.text),
            )
            .title_bottom(format!(
                "Use {} to move, r to reset position",
                if self.config.nerd_fonts {
                    "   "
                } else {
                    "↑ ↓ < >"
                }
            ));

        Paragraph::new(
            self.buffered_render
                .clone()
                .unwrap_or_else(|| "Failed rendering article".to_string()),
        )
        .scroll(self.scroll_number.clone().into())
        .wrap(Wrap { trim: true })
        .style(self.theme.text)
        .block(init_block)
        .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let mut base_block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title_top(
                Line::from(format!(
                    "{} Esc",
                    if self.config.nerd_fonts { "" } else { "x" }
                ))
                .left_aligned(),
            )
            .title_position(ratatui::widgets::block::Position::Top)
            .title_alignment(ratatui::layout::Alignment::Center);

        if self.feeds.get(self.selected_feed_idx + 1).is_some() {
            base_block = base_block.title_top(
                Line::from(format!(
                    "Next {}",
                    if self.config.nerd_fonts { "" } else { ">" }
                ))
                .right_aligned(),
            );
        }
        if self.selected_feed_idx.checked_sub(1).is_some()
            && self.feeds.get(self.selected_feed_idx - 1).is_some()
        {
            base_block = base_block.title_top(
                Line::from(format!(
                    "{} Prev",
                    if self.config.nerd_fonts { "" } else { "<" }
                ))
                .left_aligned(),
            );
        }

        if let Some(current_feed) = self.feeds[self.selected_feed_idx].clone() {
            let loaded_rss_block = base_block
                .style(self.theme.accent)
                .title(
                    Line::from(if let Some(custom_title) = current_feed.name {
                        custom_title
                    } else {
                        current_feed.feed.clone().title.unwrap().content
                    })
                    .style(self.theme.text),
                )
                .title_bottom(format!(
                    "Use {} to move, r to reset selection, <Enter> to select",
                    if self.config.nerd_fonts {
                        " "
                    } else {
                        "↑ ↓"
                    }
                ));

            let raw_list: Vec<ListItem> = current_feed
                .filtered_entries
                .iter()
                .map(|e| {
                    ListItem::from(format!(
                        "{} {} {}{} {} {}",
                        if self.config.nerd_fonts {
                            "󰃭"
                        } else {
                            "📅"
                        },
                        e.updated.unwrap().date_naive(),
                        match self.config.list_format {
                            ListFormat::Compact => "",
                            ListFormat::Extended => "\n",
                        },
                        if self.config.nerd_fonts {
                            "󰦨"
                        } else {
                            "📜"
                        },
                        e.title.clone().unwrap().content,
                        match self.config.list_format {
                            ListFormat::Compact => "",
                            ListFormat::Extended => "\n\n",
                        }
                    ))
                })
                .collect();

            let rss_list = List::new(raw_list)
                .highlight_symbol(if self.config.nerd_fonts { "❯" } else { ">" })
                .style(self.theme.accent)
                .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
                .highlight_style(self.theme.text.bold())
                .direction(ratatui::widgets::ListDirection::TopToBottom)
                .block(loaded_rss_block);

            StatefulWidget::render(rss_list, area, buf, &mut self.list_state);
        } else {
            let failure_block = base_block
                .title(Line::from("Error").centered().style(self.theme.text))
                .style(self.theme.error);

            Paragraph::new(Line::from("Failed fetching RSS information"))
                .block(failure_block)
                .render(area, buf);
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) {
        while !self.exit {
            terminal
                .draw(|frame| frame.render_widget(&mut self, frame.area()))
                .unwrap();
            if let Event::Key(key) = event::read().unwrap() {
                match self.screen {
                    CurrentScreen::Selection => self.handle_key_selection(key),
                    CurrentScreen::SingleArticle => self.handle_key_article(key),
                }
            };
        }
    }

    fn handle_key_article(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.screen = CurrentScreen::Selection;
                self.selected_entry = None;
                self.scroll_number.reset();
            }
            KeyCode::Char('r') => self.scroll_number.reset(),
            KeyCode::PageUp => {
                self.scroll_number
                    .0
                    .scroll(10, ScrollDirectionVertical::Up)
                    .ok();
            }
            KeyCode::PageDown => {
                self.scroll_number
                    .0
                    .scroll(10, ScrollDirectionVertical::Down)
                    .ok();
            }
            KeyCode::Up => {
                self.scroll_number
                    .0
                    .scroll(1, ScrollDirectionVertical::Up)
                    .ok();
            }
            KeyCode::Down => {
                self.scroll_number
                    .0
                    .scroll(1, ScrollDirectionVertical::Down)
                    .ok();
            }
            KeyCode::Left => {
                self.scroll_number
                    .1
                    .scroll(1, ScrollDirectionHorizontal::Left)
                    .ok();
            }
            KeyCode::Right => {
                self.scroll_number
                    .1
                    .scroll(1, ScrollDirectionHorizontal::Right)
                    .ok();
            }

            _ => {}
        }
    }

    fn handle_key_selection(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Left => {
                if self.selected_feed_idx.checked_sub(1).is_none() {
                    return;
                }

                self.list_state.select(None);
                self.selected_feed_idx -= 1;
            }
            KeyCode::Right => {
                if self.selected_feed_idx + 1 >= self.feeds.len() {
                    return;
                }

                self.list_state.select(None);
                self.selected_feed_idx += 1;
            }
            KeyCode::Char('r') => self.list_state.select(None),
            KeyCode::Enter => {
                if self.list_state.selected().is_none() {
                    return;
                }
                self.screen = CurrentScreen::SingleArticle;
                self.selected_entry = Some(
                    self.feeds[self.selected_feed_idx]
                        .clone()
                        .unwrap()
                        .filtered_entries[self.list_state.selected().unwrap()]
                    .clone(),
                );

                // TODO: very ugly and should be fixed
                let strbuf = self
                    .selected_entry
                    .clone()
                    .unwrap()
                    .content
                    .unwrap()
                    .body
                    .unwrap();

                let cursor = Cursor::new(strbuf);

                let mut readval =
                    html2text::from_read(cursor, Self::LARGE_NUMBER).expect("Failed reading HTML");

                if let Some(renderer) = &self.config.renderer {
                    let mut renderer_command = Command::new(renderer.binary.clone())
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .args(renderer.args.clone().unwrap_or_else(std::vec::Vec::new))
                        .spawn()
                        .expect("Failure running renderer command");

                    renderer_command
                        .stdin
                        .as_mut()
                        .unwrap()
                        .write_all(readval.as_bytes())
                        .unwrap();

                    readval =
                        String::from_utf8(renderer_command.wait_with_output().unwrap().stdout)
                            .unwrap();
                }

                self.buffered_render = Some(readval);
            }
            _ => {}
        }
    }
}
