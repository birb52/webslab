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
   git clone [https://github.com/your-username/webslab.git](https://github.com/your-username/webslab.git)

   cd webslab
   ```
    Build the project:

    ```bash
    cargo build --release
    ```
    Create a config.toml in the project root.

## Configuration

Create a config.toml file to customize the behavior:

    start_url = "index.html" # Can be a local file or a URL like "[https://google.com](https://google.com)"
    search_url = "[https://www.google.com/search?q=](https://www.google.com/search?q=)"
    icon = "assets/icon.png" # Optional
    frameless = true
    transparent = true
    always_on_top = false

## Usage

Once built, you can run the binary:
``` bash
./target/release/webslab
```

The default index.html acts as your "New Tab" page. Type a URL to navigate or a search term to use the search engine defined in your script.

## Contributing

This is a "tiny" project by design. Feel free to fork it and add your own CSS/JS to the index.html to create your perfect start page!