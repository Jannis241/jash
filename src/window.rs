use egui::FontDefinitions;

use crate::{commands, prelude::*};
pub enum CursorType {
    Block,
    Strich,
    Underline,
}

pub struct Config {
    pub font: String,
    pub zoom_level: f32,
    pub font_size: f32,
    pub background_color: (u8, u8, u8),
    pub transparency: f32,
    pub font_color: (u8, u8, u8),
    pub prompt_color: (u8, u8, u8),
    pub prompt: String,
    pub cursor_type: CursorType,
    pub time_till_cursor_starts_blinking: u128, // ms
    pub cursor_blinking_time: f32,              // s
}

impl Default for Config {
    fn default() -> Self {
        Config {
            font: "JetBrainsMono".to_string(),
            zoom_level: 1.0,
            font_size: 20.0,
            background_color: (10, 10, 10),
            transparency: 1.0,
            prompt_color: (10, 10, 10),
            font_color: (200, 200, 200),
            prompt: "$ ".to_string(),
            cursor_type: CursorType::Block,
            time_till_cursor_starts_blinking: 800,
            cursor_blinking_time: 0.6,
        }
    }
}
fn float_to_u8(value: f32) -> u8 {
    let clamped = value.clamp(0.0, 1.0);
    (clamped * 255.0).round() as u8
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
            override_text_color: None,
            window_fill: egui::Color32::from_rgba_unmultiplied(
                config.background_color.0,
                config.background_color.1,
                config.background_color.2,
                float_to_u8(config.transparency),
            ),
            panel_fill: egui::Color32::from_rgba_unmultiplied(
                config.background_color.0,
                config.background_color.1,
                config.background_color.2,
                float_to_u8(config.transparency),
            ),
            ..Default::default()
        });
        cc.egui_ctx.set_zoom_factor(config.zoom_level);

        let mut fonts = FontDefinitions::default();

        let font_name = &config.font;

        let (font_bytes, used_font_name) = match font_name.as_str() {
            "JetBrainsMono" => (
                include_bytes!("../Fonts/JetBrainsMono.ttf").to_vec(),
                "JetBrainsMono",
            ),
            _ => {
                eprintln!(
                    "Warning: Font {} not found. Falling back to JetBrainsMono.",
                    font_name
                );
                (
                    include_bytes!("../Fonts/JetBrainsMono.ttf").to_vec(),
                    "JetBrainsMono",
                )
            }
        };

        fonts.font_data.insert(
            used_font_name.to_owned(),
            egui::FontData::from_owned(font_bytes).into(),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, used_font_name.to_owned());

        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push(used_font_name.to_owned());

        cc.egui_ctx.set_fonts(fonts);
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::new(config.font_size, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(config.font_size, egui::FontFamily::Monospace),
        );
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(
            config.font_color.0,
            config.font_color.1,
            config.font_color.2,
        ));

        cc.egui_ctx.set_style(style);

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

                            if self.current_input.len() > 0 {
                                let output = commands::handle(&self.current_input);
                                self.history.push(output);
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
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
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
