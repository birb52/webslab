use std::error::Error;
use std::path::Path;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

mod config;
use config::Config;

mod icon;
use crate::icon::load_icon;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Load Config (Requires serde)
    let config = Config::load()?;
    let event_loop = EventLoop::new();

    // 2. Setup Window with Icon logic
    let mut window_builder = WindowBuilder::new()
        .with_title("webslab")
        .with_decorations(!config.frameless)
        .with_transparent(config.transparent)
        .with_always_on_top(config.always_on_top);

    if let Some(icon_path) = &config.icon {
        if let Ok(icon) = load_icon(icon_path) {
            window_builder = window_builder.with_window_icon(Some(icon));
        }
    }

    let window = window_builder.build(&event_loop)?;
    
    // 3. URL Logic
    let start_url = if !config.start_url.is_empty() && Path::new(&config.start_url).exists() {
        let abs_path = std::fs::canonicalize(&config.start_url)?;
        let path_str = abs_path.to_string_lossy();
        let clean_path = path_str.strip_prefix(r"\\?\").unwrap_or(&path_str);
        
        url::Url::from_file_path(clean_path)
            .map(|u| u.to_string())
            .unwrap_or_else(|_| "about:blank".to_string())
    } else if !config.start_url.is_empty() {
        config.start_url.clone()
    } else {
        // Default to local index.html in assets directory
        let assets_path = std::env::current_dir()?.join("assets").join("index.html");
        if assets_path.exists() {
            url::Url::from_file_path(&assets_path)
                .map(|u| u.to_string())
                .unwrap_or_else(|_| "about:blank".to_string())
        } else {
            "about:blank".to_string()
        }
    };

    let initialization_script = format!(
        "window.SEARCH_URL = '{}';", 
        config.search_url
    );

    // 4. WebView Builder
    let webview_builder = WebViewBuilder::new()
        .with_url(&start_url)
        .with_initialization_script(&initialization_script);

    let _webview = webview_builder.build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}