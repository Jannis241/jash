use crate::{commands, prelude::*};
enum CursorType {
    Block,
    Strich,
    Underline,
}

pub struct Config {
    font: String,
    zoom_level: f32,
    background_color: (u8, u8, u8),
    transparency: f32,
    font_color: (u8, u8, u8),
    prompt: String,
    cursor_type: CursorType,
    time_till_cursor_starts_blinking: u128, // ms
    cursor_blinking_time: f32,              // s
}

impl Default for Config {
    fn default() -> Self {
        Config {
            font: "JetBrains Mono".to_string(),
            zoom_level: 25.0,
            background_color: (10, 10, 10),
            transparency: 1.0,
            font_color: (255, 255, 255),
            prompt: "$ ".to_string(),
            cursor_type: CursorType::Strich,
            time_till_cursor_starts_blinking: 800,
            cursor_blinking_time: 0.6,
        }
    }
}

struct TerminalWindow {
    config: Config,
    history: Vec<String>,
    current_input: String,
    last_time_writing: Instant,
    cursor_visible: bool,
    last_cursor_toggle: Instant,
}

impl TerminalWindow {
    fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgb(
                config.background_color.0,
                config.background_color.1,
                config.background_color.2,
            ),
            ..Default::default()
        });
        cc.egui_ctx.set_zoom_factor(config.zoom_level / 10.0);

        let now = Instant::now();
        Self {
            config,
            history: Vec::new(),
            current_input: String::new(),
            last_time_writing: now,
            cursor_visible: true,
            last_cursor_toggle: now,
        }
    }
}

impl eframe::App for TerminalWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Input abfangen
        ctx.input(|input| {
            for event in &input.events {
                match event {
                    egui::Event::Text(text) => {
                        self.last_time_writing = Instant::now();
                        self.current_input.push_str(text);
                        self.cursor_visible = true; // Cursor sofort sichtbar beim Tippen
                        self.last_cursor_toggle = Instant::now();
                    }
                    egui::Event::Key { key, pressed, .. } if *pressed => match key {
                        egui::Key::Backspace => {
                            self.current_input.pop();
                            self.last_time_writing = Instant::now();
                            self.cursor_visible = true;
                            self.last_cursor_toggle = Instant::now();
                        }
                        egui::Key::Enter => {
                            self.history
                                .push(format!("{} {}", self.config.prompt, self.current_input));

                            let output = commands::handle(&self.current_input);
                            if let Some(o) = output {
                                self.history.push(o);
                            }
                            self.current_input.clear();
                            self.last_time_writing = Instant::now();
                            self.cursor_visible = true;
                            self.last_cursor_toggle = Instant::now();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        });

        // Cursor-Logik
        let now = Instant::now();
        let time_since_last_write = now - self.last_time_writing;
        if time_since_last_write.as_millis() >= self.config.time_till_cursor_starts_blinking as u128
            && self.config.cursor_blinking_time > 0.0
        {
            if (now - self.last_cursor_toggle).as_secs_f32() >= self.config.cursor_blinking_time {
                self.cursor_visible = !self.cursor_visible;
                self.last_cursor_toggle = now;
            }
        } else {
            self.cursor_visible = true; // Immer sichtbar beim Tippen oder kurz danach
        }

        // Terminal-Feld
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for line in &self.history {
                        ui.label(line);
                    }

                    let cursor_icon = match self.config.cursor_type {
                        CursorType::Block => "█",
                        CursorType::Strich => "|",
                        CursorType::Underline => "_",
                    };

                    let cursor = if self.cursor_visible {
                        cursor_icon
                    } else {
                        " "
                    };

                    ui.label(format!(
                        "{} {}{}",
                        self.config.prompt, self.current_input, cursor
                    ));
                });
        });

        ctx.request_repaint(); // UI muss jedes Frame neu zeichnen
    }
}
pub fn create_terminal_window(config: Config) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Jash",
        options,
        Box::new(|cc| Ok(Box::new(TerminalWindow::new(cc, config)))),
    )
}
