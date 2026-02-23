#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;
#[cfg(target_os = "linux")]
use gtk::prelude::*;
mod config;
use config::Config;
mod icon;
use crate::icon::load_icon;
mod adblocker;
use crate::adblocker::get_adblocker_script;

fn main() {
    // 1. Load Config (Requires serde)
    let config = Config::load().expect("Failed to load config");

    #[cfg(target_os = "linux")]
    {
        // Initialize GTK
        gtk::init().expect("Failed to initialize GTK");
    }

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

    let window = window_builder.build(&event_loop).expect("Failed to create window");
    
    // 3. URL Logic
    let start_url = if !config.start_url.is_empty() && Path::new(&config.start_url).exists() {
        let abs_path = std::fs::canonicalize(&config.start_url).expect("Failed to canonicalize path");
        let path_str = abs_path.to_string_lossy();
        let clean_path = path_str.strip_prefix(r"\\?\").unwrap_or(&path_str);
        
        url::Url::from_file_path(clean_path)
            .map(|u| u.to_string())
            .unwrap_or_else(|_| "about:blank".to_string())
    } else if !config.start_url.is_empty() {
        config.start_url.clone()
    } else {
        // Default to local index.html in assets directory
        let assets_path = std::env::current_dir().expect("Failed to get current directory").join("assets").join("index.html");
        if assets_path.exists() {
            url::Url::from_file_path(&assets_path)
                .map(|u| u.to_string())
                .unwrap_or_else(|_| "about:blank".to_string())
        } else {
            "about:blank".to_string()
        }
    };

    let adblocker_script = get_adblocker_script(
        config.adblocker_enabled,
        &config.adblocker_whitelist,
        &config.adblocker_blacklist
    );

    let initialization_script = format!(
        "window.SEARCH_URL = '{}';
        
        // Add keyboard shortcuts
        document.addEventListener('keydown', function(e) {{
            if (e.ctrlKey) {{
                switch(e.key.toLowerCase()) {{
                    case 'h':
                        e.preventDefault();
                        alert('Shortcuts:\\nCtrl+H - Show this menu\\nCtrl+F - Go forward\\nCtrl+B - Go back\\nCtrl+R - Reload page');
                        break;
                    case 'f':
                        e.preventDefault();
                        history.forward();
                        break;
                    case 'b':
                        e.preventDefault();
                        history.back();
                        break;
                    case 'r':
                        e.preventDefault();
                        location.reload();
                        break;
                }}
            }}
        }});{}",
        config.search_url,
        adblocker_script
    );

    // 4. WebView Builder
    let webview_builder = WebViewBuilder::new()
        .with_url(&start_url)
        .with_initialization_script(&initialization_script);

    let _webview = webview_builder.build(&window).expect("Failed to create webview");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        
        // Advance GTK event loop on Linux
        #[cfg(target_os = "linux")]
        while gtk::events_pending() {
            let _ = gtk::main_iteration_do(false);
        }
        
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}