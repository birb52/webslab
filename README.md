<div style="text-align: center;">
  <img src="assets/icon.png" alt="webslab logo" width="100" height="100">
</div>

# webslab

A lightweight, minimalist webview wrapper written in Rust. Designed specifically for tiling window managers like **i3, Hyprland, Sway, and Komorebi**, where you want a browser that acts like a simple window rather than a bloated desktop environment.

## Features

- **Tiling Friendly**: Supports frameless and transparent windows via configuration.
- **Ultra Minimal**: No tabs, no bookmarks, just a webview.
- **Smart Omnibox**: A custom `index.html` provides a simple search/URL bar.
- **Fast**: Built on `wry` (using the system's native web engine).

## Getting Started

### Prerequisites

Ensure you have the required dependencies for `wry` installed on your system (especially on Linux/Webkit2gtk).

## Installation

1. Clone the repository:
   ```bash
   git clone [https://github.com/birb52/webslab.git](https://github.com/birb52/webslab.git)

   cd webslab
   ```
    Build the project:

    ```bash
    cargo build --release
    ```

## Configuration

The application automatically creates a configuration file on first run. The config file location depends on your operating system:

- **Windows**: `%APPDATA%\Roaming\webslab\config.toml`
- **Linux**: `~/.config/webslab/config.toml`
- **macOS**: `~/Library/Application Support/webslab/config.toml`

The default configuration includes:
```toml
start_url = ""                    # Empty defaults to assets/index.html
search_url = ""                   # Custom search engine URL
icon = "assets/icon.png"          # Window icon path
frameless = true                  # Remove window decorations
transparent = true                # Transparent window background
always_on_top = false             # Keep window on top
```

You can customize these settings by editing the config file after it's created.

## Usage

Once built, you can run the binary:
``` bash
./target/release/webslab
```
You can use ```ctrl + h``` for the keyboard shortcuts popup guide.

The default ```index.html``` acts as your "New Tab" page. Type a URL to navigate or a search term to use the search engine defined in your script.

## Contributing

This is a "tiny" project by design. Feel free to fork it and add your own CSS/JS to the index.html to create your perfect start page!
