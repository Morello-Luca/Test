use eframe::egui;
use rust_ai_app::app_status;
use rust_ai_app::config::AppConfig;
use rust_ai_app::gui::RustAiApp;

fn main() -> eframe::Result<()> {
    println!("{}", app_status());

    let config = AppConfig::default();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.window_width as f32, config.window_height as f32])
            .with_title("RustAI Native Desktop Shell"),
        ..Default::default()
    };

    eframe::run_native(
        "RustAI",
        native_options,
        Box::new(|cc| Ok(Box::new(RustAiApp::new(cc)))),
    )
}
