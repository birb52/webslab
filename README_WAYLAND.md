# Wayland Support for Webslab

## Issue
On Wayland systems (like Fedora 43), the application may show "UnsupportedWindowHandle" error or display a grey window.

## Solution
The code has been updated to properly support Wayland by:

1. **GTK Integration**: Added GTK dependencies and initialization for Linux
2. **Event Loop Handling**: Added GTK event loop advancement in the main event loop
3. **Environment Variables**: Use the following environment variables if needed:

```bash
export GDK_BACKEND=x11
export WEBKIT_DISABLE_DMABUF_RENDERER=1
```

## Dependencies for Fedora
Install the required development packages:

```bash
sudo dnf install gtk3-devel webkit2gtk4.1-devel
```

## Building and Running
```bash
cargo build --release
cargo run
```

If issues persist, try with X11 backend:
```bash
GDK_BACKEND=x11 WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo run
```

## Technical Details
- Uses wry with GTK feature enabled
- Initializes GTK before creating the window
- Advances GTK event loop alongside tao event loop
- Properly handles window creation for both X11 and Wayland
