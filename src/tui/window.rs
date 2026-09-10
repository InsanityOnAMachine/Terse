use ratatui::text::Line;
use ratatui::layout::{Offset, Size};
use ratatui::text::Text;
use crate::tui;

use ratatui::prelude::{Widget, Buffer, Rect};
use ratatui::widgets::Block;
use ratatui::style::{Modifier, Style, Stylize};
use crossterm::event::KeyEvent;

use anyhow::Error;

pub trait Window {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), Error> {Ok(())}
    fn update(&mut self) -> Result<(), Error> {Ok(())}
    fn render(&mut self, area: Rect, buf: &mut Buffer);
    fn render_with_help(&mut self, area: Rect, buf: &mut Buffer, labels: Vec<String>) {
        self.render(area, buf);
        let key_bindings = labels.join("-");
        Text::from(key_bindings.as_str()).light_red().render(area.offset(Offset {x: 1, y: 0}).resize(Size::new(key_bindings.len() as u16, 1)), buf)
    }
    fn render_greyed_out(&mut self, area: Rect, buf: &mut Buffer, key_binding: &(impl AsRef<str> + ?Sized)) {
        self.render(area, buf);
        buf.set_style(area, Style::new().gray());

        Text::from(key_binding.as_ref()).light_red().render(area.offset(Offset {x: 1, y: 0}).resize(Size::new(key_binding.as_ref().len() as u16, 1)), buf)
    }
}

pub trait FramedWindow: Window {
    fn render_selected(&mut self, area: Rect, buf: &mut Buffer, labels: &mut Vec<String>) {
        labels.append(&mut Self::get_labels());
        self.render_in_block(area, buf, tui::get_default_block()
            .title_bottom(labels.join("-")));
    }

    fn render_unselected(&mut self, area: Rect, buf: &mut Buffer, message: &(impl AsRef<str> + ?Sized)) {
        self.render_in_block(area, buf, tui::get_default_block());
        buf.set_style(area, Style::new().gray());

        Text::from(message.as_ref()).light_red().render(area.offset(Offset {x: 1, y: 0}).resize(Size {width: message.as_ref().len() as u16, height: 1}), buf)
    }

    fn render_in_block(&mut self, area: Rect, buf: &mut Buffer, block: Block) {
        let inner = block.inner(area);
        block.render(area, buf);
        self.render(inner, buf);
    }

    fn get_labels() -> Vec<String> {vec![]}
}

impl<T> Window for Option<T> where T: Window {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        match self {
            Some(x) => x.render(area, buf),
            _ => {}
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), Error> {
        match self {
            Some(x) => x.handle_key_event(key),
            _ => Ok(())
        }
    }

    fn update(&mut self) -> Result<(), Error> {
        match self {
            Some(x) => x.update(),
            _ => Ok(())
        }
    }
}

impl<T> FramedWindow for Option<T> where T: FramedWindow {
    fn render_selected(&mut self, area: Rect, buf: &mut Buffer, labels: &mut Vec<String>) {
        match self {
            Some(window) => {window.render_selected(area, buf, labels)},
            None => {tui::get_default_block().render(area, buf)}
        }
    }

    fn render_unselected(&mut self, area: Rect, buf: &mut Buffer, message: &(impl AsRef<str> + ?Sized)) {
        match self {
            Some(window) => {window.render_unselected(area, buf, &message)},
            None => {tui::get_default_block().border_style(Style::new().gray()).render(area, buf)}
        }
    }
}

impl<T> Window for Result<T, anyhow::Error> where T: Window {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        match self {
            Ok(x) => x.render(area, buf),
            Err(e) => {Line::from(format!("(!) There was an error: {} (!)", e)).centered().red().render(area, buf)}
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), Error> {
        match self {
            Ok(x) => x.handle_key_event(key),
            _ => Ok(())
        }
    }

    fn update(&mut self) -> Result<(), Error> {
        match self {
            Ok(x) => x.update(),
            _ => Ok(())
        }
    }
}

impl<T> FramedWindow for Result<T, anyhow::Error> where T: Window {}