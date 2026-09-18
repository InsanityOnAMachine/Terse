use ratatui::widgets::Padding;
use ratatui::layout::{Offset, Size};
use ratatui::text::Text;
use crate::tui;

use ratatui::prelude::{Widget, Buffer, Rect};
use ratatui::widgets::{Paragraph};
use ratatui::style::{Color, Style, Stylize};
use crossterm::event::KeyEvent;

use anyhow::Error;

/// The Component struct is basically a component;
/// wow. That comment was left over from when this struct was called Window.
///
/// It has:
/// - key event handling, which returns a Result of the defined type Action to report to its parent
///   (by default (), and by the way, trait type defaults need a nightly feature flag right now)
///
/// - rendering support as a wrapper for ratatui's Widget, along with 
/// - render_with_help() to have bottom keybinding instructions with the render
/// - render_greyed_out() for deselected Components, with an optional switch-key
/// - get_labels() for any self-defined labels to be taken into account in render_with_help() besides the provided arguments
/// - select() and deselect() for any on-leave action (which should be called by the parent and so on recursively)
///
/// Component is also implemented for Option<Component> and Result<Component>, and of course recursive containment is supported
pub trait Component {
    /// The type that is returned (in a Result along with anyhow::Error) from handle_key_event(), by default ()
    type Action = ();
    /// Processes a single key event, returning a Result<Self::Action, anyhow::Error>
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Self::Action, Error>;
    //fn update(&mut self) -> Result<(), Error> {Ok(())}
    fn render(&mut self, area: Rect, buf: &mut Buffer, selected: bool);
    fn render_with_help(&mut self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf, true);
        let key_bindings = self.get_labels().join("-");
        Text::from(key_bindings.as_str()).light_red().render(area.offset(Offset {x: 1, y: (area.height - 1).into()}).resize(Size::new(key_bindings.len() as u16, 1)), buf)
    }
    fn render_greyed_out(&mut self, area: Rect, buf: &mut Buffer, key_binding: &(impl AsRef<str> + ?Sized)) {
        self.render(area, buf, false);

        // We yank the code for Buffer::set_style() because we only want to set the bg style if it is not empty
        // https://docs.rs/ratatui-core/0.1.2/src/ratatui_core/buffer/buffer.rs.html#405

        let area = buf.area.intersection(area);
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let cell = &mut buf[(x, y)];

                cell.set_fg(Color::Gray);
                if cell.style().bg.is_some() && cell.style().bg != Some(Color::Reset) {
                    cell.set_bg(Color::Gray);
                }
            }
        }

        Text::from(key_binding.as_ref()).light_red().render(area.offset(Offset {x: 1, y: 0}).resize(Size::new(key_binding.as_ref().len() as u16, 1)), buf)
    }
    fn get_labels(&self) -> Vec<String> {vec![]}
    fn select(&mut self) {}
    fn deselect(&mut self) {}
}

impl<T> Component for Option<T> where T: Component {
    type Action = Option<T::Action>;
    fn render(&mut self, area: Rect, buf: &mut Buffer, selected: bool) {
        match self {
            Some(x) => x.render(area, buf, selected),
            _ => {
                // https://www.reddit.com/r/learnrust/comments/16ibtin/centring_text_in_ratatui/
                Paragraph::new("Nothing here!").gray().centered()
                .block(tui::get_default_block().padding(Padding::new(
                    0, // left
                    0, // right
                    area.height / 2, // top
                    0, // bottom
                ))).render(area, buf)
            }
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Self::Action, Error> {
        match self {
            Some(x) => Some(x.handle_key_event(key)).transpose(),
            _ => Ok(None)
        }
    }

    fn render_with_help(&mut self, area: Rect, buf: &mut Buffer) {
        match self {
            Some(window) => {window.render_with_help(area, buf)},
            None => {self.render(area, buf, true)}
        }
    }

    fn render_greyed_out(&mut self, area: Rect, buf: &mut Buffer, message: &(impl AsRef<str> + ?Sized)) {
        match self {
            Some(window) => {window.render_greyed_out(area, buf, &message)},
            None => {self.render(area, buf, false); buf.set_style(area, Style::new().gray());}
        }
    }
    fn select(&mut self) {
        match self {
            Some(window) => {window.select()},
            _ => {}
        }
    }
    fn deselect(&mut self) {
        match self {
            Some(window) => {window.deselect()},
            _ => {}
        }
    }
}

impl<T> Component for Result<T, anyhow::Error> where T: Component {
    type Action = Option<T::Action>;

    fn render(&mut self, area: Rect, buf: &mut Buffer, selected: bool) {
        match self {
            Ok(x) => x.render(area, buf, selected),
            Err(e) => {
                // https://www.reddit.com/r/learnrust/comments/16ibtin/centring_text_in_ratatui/
                Paragraph::new(format!("(!) There was an error: {} (!)", e)).red().centered()
                .block(tui::get_default_block().padding(Padding::new(
                    0, // left
                    0, // right
                    area.height / 2, // top
                    0, // bottom
                ))).render(area, buf);
            }
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Self::Action, Error> {
        match self {
            Ok(x) => Some(x.handle_key_event(key)).transpose(),
            _ => Ok(None)
        }
    }

    fn render_with_help(&mut self, area: Rect, buf: &mut Buffer) {
        match self {
            Ok(window) => {window.render_with_help(area, buf)},
            Err(_) => {self.render(area, buf, true)}
        }
    }

    fn render_greyed_out(&mut self, area: Rect, buf: &mut Buffer, message: &(impl AsRef<str> + ?Sized)) {
        match self {
            Ok(window) => {window.render_greyed_out(area, buf, &message)},
            Err(_) => {self.render(area, buf, false); buf.set_style(area, Style::new().gray());}
        }
    }
    fn select(&mut self) {
        match self {
            Ok(window) => {window.select()},
            _ => {}
        }
    }
    fn deselect(&mut self) {
        match self {
            Ok(window) => {window.deselect()},
            _ => {}
        }
    }
}
