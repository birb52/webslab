use std::{error::Error, path::PathBuf};
use tao::window::Icon;

pub fn load_icon(path: &str) -> Result<Icon, Box<dyn Error>> {
    // Resolve path relative to the project root (workspace directory)
    let project_root = std::env::current_dir()?;
    let full_path: PathBuf = project_root.join(path);

    let image = image::open(&full_path)?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    Ok(Icon::from_rgba(rgba, width, height)?)
}