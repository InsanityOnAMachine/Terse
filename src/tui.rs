pub mod app;
pub use app::*;


pub mod label;
pub use label::*;

pub mod size;
pub use size::*;

pub mod component;
pub use component::*;

#[cfg(debug_assertions)]
pub mod blinker;
#[cfg(debug_assertions)]
pub use blinker::*;

pub mod search_menu;
pub use search_menu::*;

pub mod server_menu;
pub use server_menu::*;

pub mod posts;
pub use posts::*;

use ratatui::widgets::{Widget, Block, BorderType};
use ratatui::style::Style;
use ratatui::layout::Rect;
use ratatui::buffer::Buffer;

pub fn get_default_block<'a>() -> Block<'a> {
    return Block::bordered()
        .border_type(BorderType::Plain)
        .border_style(Style::new().light_red())
        //.border_style(Style::new().white().on_white())
}

/// Renders the Default Terse Red Block in the given area and returns the inner area for you to
/// render the rest of your whatever inside of.
pub fn block_and_inner(area: Rect, buf: &mut Buffer) -> Rect {
    let block = get_default_block();
    let inner = block.inner(area);
    block.render(area, buf);
    inner
}
