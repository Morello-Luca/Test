use rust_ai_app::app_status;
use rust_ai_app::config::AppConfig;

fn main() {
    println!("{}", app_status());
    let default_config = AppConfig::default();
    println!(
        "Application Window Configured: {}x{} | Theme: {:?}",
        default_config.window_width, default_config.window_height, default_config.theme
    );
}
