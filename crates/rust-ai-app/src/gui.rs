use crate::config::{AppConfig, ThemeMode};
use eframe::egui;
use rust_ai_core::{
    discovery::{DiscoveredModel, ModelDiscovery},
    traits::{ConversationMessage, ModelLoadState, ModelProvider},
};
use rust_ai_providers::LlamaCppBackend;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

enum BackgroundTask {
    LoadModel(PathBuf, String),
    UnloadModel,
    Generate(String),
}

enum BackgroundEvent {
    LoadStarted(String),
    LoadFinished(Result<String, String>),
    UnloadFinished,
    GenerateStarted,
    GenerateFinished(Result<String, String>),
}

pub struct RustAiApp {
    pub config: AppConfig,
    pub discovered_models: Vec<DiscoveredModel>,
    pub selected_model_index: Option<usize>,
    pub input_prompt: String,
    pub messages: Vec<ConversationMessage>,
    pub status_message: Option<String>,
    pub model_load_state: ModelLoadState,
    pub is_generating: bool,

    tx_task: Sender<BackgroundTask>,
    rx_event: Receiver<BackgroundEvent>,
}

impl RustAiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let config = AppConfig::default();

        if let Err(e) = std::fs::create_dir_all(&config.models_dir) {
            eprintln!("Warning: Failed to create models directory: {}", e);
        }

        let discovered_models =
            ModelDiscovery::scan_directory(&config.models_dir).unwrap_or_default();

        let selected_model_index = if !discovered_models.is_empty() {
            Some(0)
        } else {
            None
        };

        let (tx_task, rx_task) = channel::<BackgroundTask>();
        let (tx_event, rx_event) = channel::<BackgroundEvent>();

        // Spawn background worker thread to handle model loading, unloading & inference without blocking GUI
        thread::spawn(move || {
            let mut backend = LlamaCppBackend::new();

            while let Ok(task) = rx_task.recv() {
                match task {
                    BackgroundTask::LoadModel(path, name) => {
                        let _ = tx_event.send(BackgroundEvent::LoadStarted(name.clone()));
                        match backend.load_model(&path) {
                            Ok(_) => {
                                let _ = tx_event.send(BackgroundEvent::LoadFinished(Ok(name)));
                            }
                            Err(e) => {
                                let _ = tx_event.send(BackgroundEvent::LoadFinished(Err(e)));
                            }
                        }
                    }
                    BackgroundTask::UnloadModel => {
                        backend.unload_model();
                        let _ = tx_event.send(BackgroundEvent::UnloadFinished);
                    }
                    BackgroundTask::Generate(prompt) => {
                        let _ = tx_event.send(BackgroundEvent::GenerateStarted);
                        let result = backend.generate_response(&prompt);
                        let _ = tx_event.send(BackgroundEvent::GenerateFinished(result));
                    }
                }
            }
        });

        Self {
            config,
            discovered_models,
            selected_model_index,
            input_prompt: String::new(),
            messages: vec![ConversationMessage::new(
                "system",
                "Welcome to RustAI! Place GGUF model files in the 'models/' directory to begin.",
            )],
            status_message: None,
            model_load_state: ModelLoadState::Unloaded,
            is_generating: false,
            tx_task,
            rx_event,
        }
    }

    pub fn refresh_models(&mut self) {
        match ModelDiscovery::scan_directory(&self.config.models_dir) {
            Ok(models) => {
                self.discovered_models = models;
                if self.selected_model_index.is_none() && !self.discovered_models.is_empty() {
                    self.selected_model_index = Some(0);
                }
                self.status_message = Some(format!(
                    "Discovered {} model(s) in '{}'.",
                    self.discovered_models.len(),
                    self.config.models_dir.display()
                ));
            }
            Err(e) => {
                self.status_message = Some(format!("Error scanning models directory: {}", e));
            }
        }
    }

    fn poll_events(&mut self) {
        while let Ok(event) = self.rx_event.try_recv() {
            match event {
                BackgroundEvent::LoadStarted(name) => {
                    self.model_load_state = ModelLoadState::Loading;
                    self.status_message = Some(format!("Loading model '{}'...", name));
                }
                BackgroundEvent::LoadFinished(Ok(name)) => {
                    self.model_load_state = ModelLoadState::Loaded(name.clone());
                    self.status_message = Some(format!("Successfully loaded model '{}'.", name));
                }
                BackgroundEvent::LoadFinished(Err(err)) => {
                    self.model_load_state = ModelLoadState::Error(err.clone());
                    self.status_message = Some(format!("Model load failed: {}", err));
                }
                BackgroundEvent::UnloadFinished => {
                    self.model_load_state = ModelLoadState::Unloaded;
                    self.status_message = Some("Model unloaded.".to_string());
                }
                BackgroundEvent::GenerateStarted => {
                    self.is_generating = true;
                }
                BackgroundEvent::GenerateFinished(Ok(response)) => {
                    self.is_generating = false;
                    self.messages
                        .push(ConversationMessage::new("assistant", response));
                }
                BackgroundEvent::GenerateFinished(Err(err)) => {
                    self.is_generating = false;
                    self.messages.push(ConversationMessage::new(
                        "system",
                        format!("Error: {}", err),
                    ));
                }
            }
        }
    }
}

impl eframe::App for RustAiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_events();

        if self.is_generating || matches!(self.model_load_state, ModelLoadState::Loading) {
            ctx.request_repaint();
        }

        // Top Menu Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Refresh Models").clicked() {
                        self.refresh_models();
                        ui.close_menu();
                    }
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

        // Left Side Panel (Model Discovery & Selection Controls)
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading("Local Models");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Models Dir:");
                    if ui.button("Refresh").clicked() {
                        self.refresh_models();
                    }
                });
                ui.label(
                    egui::RichText::new(self.config.models_dir.to_string_lossy())
                        .small()
                        .monospace(),
                );

                ui.add_space(8.0);
                ui.separator();

                if self.discovered_models.is_empty() {
                    ui.group(|ui| {
                        ui.label(
                            egui::RichText::new("No GGUF models found.")
                                .strong()
                                .color(egui::Color32::RED),
                        );
                        ui.add_space(4.0);
                        ui.label("To load a local model:");
                        ui.label("1. Download a .gguf model file.");
                        ui.label(format!(
                            "2. Save it into '{}'.",
                            self.config.models_dir.display()
                        ));
                        ui.label("3. Click 'Refresh' above.");
                    });
                } else {
                    ui.label("Discovered Models:");
                    for (idx, model) in self.discovered_models.iter().enumerate() {
                        let size_mb = model.size_bytes / (1024 * 1024);
                        let selected = self.selected_model_index == Some(idx);
                        if ui
                            .selectable_label(selected, format!("{} ({} MB)", model.name, size_mb))
                            .clicked()
                        {
                            self.selected_model_index = Some(idx);
                        }
                    }
                }

                ui.add_space(10.0);
                ui.separator();

                // Model Loading & Switching Controls
                ui.label("Model Lifecycle:");
                ui.label(format!("Status: {}", self.model_load_state));

                ui.add_space(4.0);
                match &self.model_load_state {
                    ModelLoadState::Unloaded | ModelLoadState::Error(_) => {
                        if let Some(idx) = self.selected_model_index {
                            if idx < self.discovered_models.len() {
                                let target_model = &self.discovered_models[idx];
                                if ui.button(format!("Load '{}'", target_model.name)).clicked() {
                                    let path = target_model.path.clone();
                                    let name = target_model.name.clone();
                                    let _ =
                                        self.tx_task.send(BackgroundTask::LoadModel(path, name));
                                }
                            }
                        }
                    }
                    ModelLoadState::Loading => {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label("Loading model into memory...");
                        });
                    }
                    ModelLoadState::Loaded(ref active_name) => {
                        ui.label(format!("Active: {}", active_name));
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            if ui.button("Unload Model").clicked() {
                                let _ = self.tx_task.send(BackgroundTask::UnloadModel);
                            }

                            if let Some(idx) = self.selected_model_index {
                                if idx < self.discovered_models.len() {
                                    let target_model = &self.discovered_models[idx];
                                    if target_model.name != *active_name
                                        && ui
                                            .button(format!("Switch to '{}'", target_model.name))
                                            .clicked()
                                    {
                                        let path = target_model.path.clone();
                                        let name = target_model.name.clone();
                                        let _ = self
                                            .tx_task
                                            .send(BackgroundTask::LoadModel(path, name));
                                    }
                                }
                            }
                        });
                    }
                }

                if let Some(ref status) = self.status_message {
                    ui.add_space(8.0);
                    ui.separator();
                    ui.label(egui::RichText::new(status).small().italics());
                }
            });

        // Central Panel (Chat Interface)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chat Session");
            ui.separator();

            // Scrollable Message History Area
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(ui.available_height() - 60.0)
                .show(ui, |ui| {
                    for msg in &self.messages {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(&msg.role).strong());
                            ui.label(&msg.content);
                        });
                        ui.add_space(4.0);
                    }
                    if self.is_generating {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label(egui::RichText::new("Model generating response...").italics());
                        });
                    }
                });

            ui.separator();

            // Bottom Input Area
            ui.horizontal(|ui| {
                let is_loaded = matches!(self.model_load_state, ModelLoadState::Loaded(_));
                let input_enabled = is_loaded && !self.is_generating;

                let response = ui.add_enabled(
                    input_enabled,
                    egui::TextEdit::singleline(&mut self.input_prompt).hint_text(
                        if self.is_generating {
                            "Generating response..."
                        } else if is_loaded {
                            "Ask local model..."
                        } else {
                            "Load a model on the left to chat..."
                        },
                    ),
                );

                let send_clicked = ui
                    .add_enabled(input_enabled, egui::Button::new("Send"))
                    .clicked();
                let enter_pressed =
                    response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if input_enabled
                    && (send_clicked || enter_pressed)
                    && !self.input_prompt.trim().is_empty()
                {
                    let prompt = std::mem::take(&mut self.input_prompt);
                    self.messages
                        .push(ConversationMessage::new("user", &prompt));

                    let _ = self.tx_task.send(BackgroundTask::Generate(prompt));
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
        let (tx_task, _) = channel();
        let (_, rx_event) = channel();

        let app = RustAiApp {
            config: AppConfig::default(),
            discovered_models: Vec::new(),
            selected_model_index: None,
            input_prompt: String::new(),
            messages: Vec::new(),
            status_message: None,
            model_load_state: ModelLoadState::Unloaded,
            is_generating: false,
            tx_task,
            rx_event,
        };
        assert_eq!(app.config.models_dir, PathBuf::from("models"));
        assert_eq!(app.model_load_state, ModelLoadState::Unloaded);
    }
}
