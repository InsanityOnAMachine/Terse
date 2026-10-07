use crate::tui::{Label, MinSize, Component};
use super::PostScroller;

use ratatui::{prelude::{Text, Widget}, style::Stylize, widgets::Clear};
use ratatui::layout::{Layout, Direction, Constraint};

use anyhow::Error;
use arboard::Clipboard;

// The PostReader wraps a PostScroller in the context of a search result. When you ask to copy text
// it pops up a message saying 'Copied!' or some error and doing anything at all will make that
// message disappear. Very decent.
pub struct PostReader {
    post_scroller: PostScroller,
    copied_message: Option<Result<(), Error>>,
}

impl PostReader {
    pub fn new(post: crate::posts::Post) -> Self {
        Self {post_scroller: PostScroller::new(post), copied_message: None}
    }
}

impl Component for PostReader {
    fn render(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, selected: bool) {
        self.post_scroller.render(area, buf, selected);

        if let Some(message) = &self.copied_message && selected {
            let message_string = match message {
                Ok(_) => {
                    String::from("Copied!")
                }
                Err(e) => {
                    String::from("I got an error: ") + &e.to_string()
                }
            };

            // The compiler doesn't complain! Bliss~
            let [_, middle, _] = Layout::default().direction(Direction::Vertical).constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ]).areas(area);
            let [_, middle, _] = Layout::default().direction(Direction::Horizontal).constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(message_string.len() as u16),
                Constraint::Fill(1),
            ]).areas(middle);

            Clear::default().render(middle, buf);
            Text::from(message_string).centered().white().on_blue().render(middle, buf);
        }
    }    
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Self::Action, anyhow::Error> {
        match key.code {
            crossterm::event::KeyCode::Char('c') => {
                match |text: String| -> Result<(), Error> {
                    let mut clipboard = Clipboard::new()?;
                    Ok(clipboard.set_text(text)?)
                }(self.post_scroller.get_post().content) {
                    Ok(_) => {self.copied_message = Some(Ok(()))},
                    Err(e) => {self.copied_message = Some(Err(e))} 
                };
                Ok(())
            }
            _ => {
                self.copied_message = None;
                self.post_scroller.handle_key_event(key)
            }
        }
    }
    fn get_labels(&self) -> Vec<String> {
        let mut labels = self.post_scroller.get_labels();
        labels.append(&mut vec![
            Label::new("c", "copy"),
        ]);
        labels
    }
    fn select(&mut self) {
        self.post_scroller.select();
    }
    fn deselect(&mut self) {
        self.post_scroller.deselect();
        self.copied_message = None;
    }
    fn get_min_size(&self) -> crate::tui::MinSize {
        let child_size = self.post_scroller.get_min_size();
        MinSize::new(child_size.width.max(self.get_text_fit_size() as u16), child_size.height)
    }
}
