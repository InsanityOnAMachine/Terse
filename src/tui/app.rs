use std::time::Duration;
use crate::tui::Component;

use ratatui::{
    prelude::Stylize, text::Text,
    DefaultTerminal, layout::{Layout, Direction, Constraint}, text::Span, widgets::Widget,
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

#[cfg(debug_assertions)]
use crate::tui::Blinker;

#[derive(Default)]
pub struct App {
    exit: bool,
    #[cfg(debug_assertions)]
    blinker: Blinker,
}

impl App {
    pub fn run<T: Component>(&mut self, window: &mut T) -> Result<(), std::io::Error> {
        ratatui::run(|terminal| self.run_loop(terminal, window))
    }

    pub fn run_loop<T: Component>(&mut self, terminal: &mut DefaultTerminal, window: &mut T) -> Result<(), std::io::Error> {
        while !self.exit {
           terminal.draw(|frame| {
                if !window.get_min_size().fits_in(frame.area()) {
                    Text::from("Too small! Make bigger!").centered().render(frame.area(), frame.buffer_mut());
                    return
                }
                // https://docs.rs/ratatui/latest/ratatui/prelude/struct.Layout.html#method.areas
                let [top, bottom] = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Length(1),
                ])
                .areas(frame.area());

                // https://stackoverflow.com/questions/30026893/how-to-use-a-map-over-vectors#30026986
                window.render(top, frame.buffer_mut(), true);

                #[cfg(debug_assertions)]
                self.blinker.render(bottom, frame.buffer_mut());
                Span::from("ESC to quit").on_red().into_right_aligned_line().render(bottom, frame.buffer_mut());

                Text::from(format!("{:?}", window.get_min_size())).render(bottom, frame.buffer_mut())
            })?;

            match event::poll(Duration::from_millis(0)) {
                Ok(true) => {
                    if let Event::Key(key_event) = event::read()? {
                        if let KeyEventKind::Press | KeyEventKind::Repeat = key_event.kind {
                            match key_event.code {
                                KeyCode::Esc => self.exit = true,
                                _ => _ = window.handle_key_event(key_event),
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
