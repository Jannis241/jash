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

    // todo:
    // - die zwei bugs fixxen
    // - mehr fonts
    // - themes?
    // - mehr farben bei zb ls oder wenn generell was ausgegben wird (so theme mäßig)
    // - prompt verbessern (zb option machen dass der path der prompt ist oder wie in zsh mit git
    // oder beides)
    // echte config datei auslesen. (vllt rust config datie machen wie jesko)

    // config.prompt_color = (255, 0, 0); // nicht implemented
    // config.transparency = 0.5; // geht nicht

    println!("Config path: {:?}", get_config_path());

    match window::create_terminal_window(config) {
        Ok(_) => println!("Successfully finished."),
        Err(e) => println!("Error with the terminal window: {}", e),
    };
}
