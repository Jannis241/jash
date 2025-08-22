use crate::prelude::*;
mod commands;
mod prelude;
mod window;

fn main() {
    // todo: config in home/.config/jash suchen oder bei appdata in windows und die dann hier rein
    // passen
    match window::create_terminal_window(window::Config::default()) {
        Ok(_) => {}
        Err(e) => println!("Error while trying to create terminal window: {}", e),
    };
}
