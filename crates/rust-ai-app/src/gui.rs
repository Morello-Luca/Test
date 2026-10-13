use crate::config::{AppConfig, ThemeMode};
use eframe::egui;

pub struct RustAiApp {
    pub config: AppConfig,
    pub input_prompt: String,
    pub messages: Vec<(String, String)>,
}

impl RustAiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set initial visuals
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            config: AppConfig::default(),
            input_prompt: String::new(),
            messages: vec![(
                "system".to_string(),
                "Welcome to RustAI! This is a native GUI desktop shell powered by egui."
                    .to_string(),
            )],
        }
    }
}

impl eframe::App for RustAiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top Menu Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Theme", |ui| {
                    if ui
                        .radio_value(&mut self.config.theme, ThemeMode::Dark, "Dark")
                        .clicked()
                    {
                        ctx.set_visuals(egui::Visuals::dark());
                    }
                    if ui
                        .radio_value(&mut self.config.theme, ThemeMode::Light, "Light")
                        .clicked()
                    {
                        ctx.set_visuals(egui::Visuals::light());
                    }
                    if ui
                        .radio_value(&mut self.config.theme, ThemeMode::System, "System")
                        .clicked()
                    {
                        ctx.set_visuals(egui::Visuals::dark());
                    }
                });
            });
        });

        // Left Side Panel (Model & Provider Controls)
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading("RustAI Panel");
                ui.separator();

                ui.label("Provider:");
                ui.text_edit_singleline(&mut self.config.default_model_provider);

                ui.add_space(5.0);
                ui.label("Model Name:");
                ui.text_edit_singleline(&mut self.config.default_model_name);

                ui.add_space(10.0);
                ui.separator();
                ui.label(format!(
                    "Window: {}x{}",
                    self.config.window_width, self.config.window_height
                ));
            });

        // Central Panel (Chat Stream Prototype)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chat Session");
            ui.separator();

            // Scrollable Message History Area
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(ui.available_height() - 60.0)
                .show(ui, |ui| {
                    for (sender, text) in &self.messages {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(sender).strong());
                            ui.label(text);
                        });
                        ui.add_space(4.0);
                    }
                });

            ui.separator();

            // Bottom Input Area
            ui.horizontal(|ui| {
                let response = ui.add_sized(
                    [ui.available_width() - 70.0, 30.0],
                    egui::TextEdit::singleline(&mut self.input_prompt).hint_text("Ask RustAI..."),
                );

                let send_clicked = ui.button("Send").clicked();
                let enter_pressed =
                    response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if (send_clicked || enter_pressed) && !self.input_prompt.trim().is_empty() {
                    let prompt = std::mem::take(&mut self.input_prompt);
                    self.messages.push(("user".to_string(), prompt.clone()));
                    self.messages.push((
                        "assistant".to_string(),
                        format!(
                            "Echo from RustAI ({}/{}): {}",
                            self.config.default_model_provider,
                            self.config.default_model_name,
                            prompt
                        ),
                    ));
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gui_app_initialization() {
        let app = RustAiApp {
            config: AppConfig::default(),
            input_prompt: String::new(),
            messages: Vec::new(),
        };
        assert_eq!(app.config.default_model_provider, "Ollama");
        assert!(app.messages.is_empty());
    }
}
