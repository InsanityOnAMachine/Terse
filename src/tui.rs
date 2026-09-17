pub mod app;
pub use app::*;

pub mod selectable;
pub use selectable::*;

pub mod label;
pub use label::*;

pub mod component;
pub use component::*;

#[cfg(debug_assertions)]
pub mod blinker;
#[cfg(debug_assertions)]
pub use blinker::*;

use ratatui::widgets::{Block, BorderType};
use ratatui::style::Style;

pub fn get_default_block<'a>() -> Block<'a> {
    return Block::bordered()
        .border_type(BorderType::Plain)
        .border_style(Style::new().light_red())
        //.border_style(Style::new().white().on_white())
}

pub fn get_selected_block<'a>() -> Block<'a> {
    return get_default_block().border_style(Style::new().light_blue())
}
