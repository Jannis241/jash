use crate::{prelude::*, window::CursorType};
mod commands;
mod prelude;
mod window;

fn main() {
    let mut config = window::Config::default();

    config.zoom_level = 2.0;
    config.font_size = 50.0;
    config.cursor_type = CursorType::Strich;

    match window::create_terminal_window(config) {
        Ok(_) => println!("Successfully finished."),
        Err(e) => println!("Error with the terminal window: {}", e),
    };
}
