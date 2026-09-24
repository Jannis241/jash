use crate::prelude::*;
mod commands;
mod prelude;
mod window;

fn get_config_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "jash").unwrap();
    proj_dirs.config_dir().to_path_buf()
}

fn main() {
    let mut config = window::Config::default();

    // config.prompt_color = (255, 0, 0);
    // config.transparency = 0.5;

    println!("Config path: {:?}", get_config_path());

    match window::create_terminal_window(config) {
        Ok(_) => println!("Successfully finished."),
        Err(e) => println!("Error with the terminal window: {}", e),
    };
}
