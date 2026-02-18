//! KDE Plasma Color Scheme Detection and Application
//!
//! This module reads KDE's kdeglobals file and Plasma color schemes to automatically
//! theme Kurrent terminal to match the desktop environment.
//!
//! ## Color Scheme Files
//!
//! - User schemes: `~/.local/share/color-schemes/*.colors`
//! - System schemes: `/usr/share/color-schemes/*.colors`
//! - Active config: `~/.config/kdeglobals`

use anyhow::Result;
use ini::configparser::ini::Ini;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// KDE Color Scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdeColorScheme {
    pub name: String,
    pub window_background: Rgb,
    pub window_foreground: Rgb,
    pub view_background: Rgb,
    pub view_foreground: Rgb,
    pub selection_background: Rgb,
    pub selection_foreground: Rgb,
    pub active_background: Rgb,
    pub active_foreground: Rgb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Parse KDE color format: "r,g,b"
    pub fn from_kde_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Self {
            r: parts[0].trim().parse().ok()?,
            g: parts[1].trim().parse().ok()?,
            b: parts[2].trim().parse().ok()?,
        })
    }
}

/// Get the current KDE color scheme by reading kdeglobals and loading the scheme file
pub fn get_current_color_scheme() -> Result<Option<KdeColorScheme>> {
    let scheme_name = match parse_kdeglobals()? {
        Some(name) => name,
        None => return Ok(None),
    };
    load_color_scheme(&scheme_name)
}

/// Get kdeglobals file path
pub fn kdeglobals_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("kdeglobals")
}

/// Parse kdeglobals INI file and return the active ColorScheme name
pub fn parse_kdeglobals() -> Result<Option<String>> {
    let path = kdeglobals_path();
    if !path.exists() {
        return Ok(None);
    }
    let mut conf = Ini::new();
    conf.load(
        path.to_str()
            .ok_or_else(|| anyhow::anyhow!("kdeglobals path contains invalid UTF-8"))?,
    )
    .map_err(|e| anyhow::anyhow!("Failed to parse kdeglobals: {}", e))?;
    Ok(conf.get("general", "colorscheme"))
}

/// Search paths for color scheme files
fn color_scheme_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(data_dir) = dirs::data_dir() {
        paths.push(data_dir.join("color-schemes"));
    }
    paths.push(PathBuf::from("/usr/share/color-schemes"));
    paths
}

/// Load a color scheme file by name, searching standard locations
pub fn load_color_scheme(name: &str) -> Result<Option<KdeColorScheme>> {
    let filename = format!("{}.colors", name);
    for dir in color_scheme_search_paths() {
        let path = dir.join(&filename);
        if path.exists() {
            return parse_color_scheme_file(&path, name).map(Some);
        }
    }
    Ok(None)
}

/// Helper to read an RGB color from a parsed INI config
fn read_color(conf: &Ini, section: &str, key: &str) -> Rgb {
    conf.get(section, key)
        .as_deref()
        .and_then(Rgb::from_kde_string)
        .unwrap_or(Rgb { r: 0, g: 0, b: 0 })
}

/// Parse a `.colors` scheme file into a `KdeColorScheme`
pub fn parse_color_scheme_file(path: &Path, name: &str) -> Result<KdeColorScheme> {
    let mut conf = Ini::new();
    conf.load(
        path.to_str()
            .ok_or_else(|| anyhow::anyhow!("Color scheme path contains invalid UTF-8: {:?}", path))?,
    )
    .map_err(|e| anyhow::anyhow!("Failed to parse color scheme {}: {}", path.display(), e))?;

    let display_name = conf
        .get("general", "name")
        .unwrap_or_else(|| name.to_string());

    Ok(KdeColorScheme {
        name: display_name,
        window_background: read_color(&conf, "colors:window", "backgroundnormal"),
        window_foreground: read_color(&conf, "colors:window", "foregroundnormal"),
        view_background: read_color(&conf, "colors:view", "backgroundnormal"),
        view_foreground: read_color(&conf, "colors:view", "foregroundnormal"),
        selection_background: read_color(&conf, "colors:selection", "backgroundnormal"),
        selection_foreground: read_color(&conf, "colors:selection", "foregroundnormal"),
        active_background: read_color(&conf, "colors:header", "backgroundnormal"),
        active_foreground: read_color(&conf, "colors:header", "foregroundnormal"),
    })
}

/// Listen for KDE color scheme changes via DBus
pub fn watch_color_scheme_changes<F>(_callback: F) -> Result<()>
where
    F: Fn(KdeColorScheme) + Send + 'static,
{
    // Watching for org.kde.KWin color scheme change signals requires
    // an async runtime. This is a placeholder for future implementation.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdeglobals_path() {
        let path = kdeglobals_path();
        assert!(path.to_string_lossy().contains("kdeglobals"));
    }

    #[test]
    fn test_rgb_parsing() {
        let rgb = Rgb::from_kde_string("255,128,64").unwrap();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 128);
        assert_eq!(rgb.b, 64);
    }

    #[test]
    fn test_rgb_parsing_with_spaces() {
        let rgb = Rgb::from_kde_string("255, 128, 64").unwrap();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 128);
        assert_eq!(rgb.b, 64);
    }

    #[test]
    fn test_rgb_parsing_invalid() {
        assert!(Rgb::from_kde_string("").is_none());
        assert!(Rgb::from_kde_string("255").is_none());
        assert!(Rgb::from_kde_string("abc,def,ghi").is_none());
        assert!(Rgb::from_kde_string("256,0").is_none());
    }

    #[test]
    fn test_parse_color_scheme_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("TestScheme.colors");
        std::fs::write(
            &path,
            "\
[General]
Name=Test Scheme

[Colors:Window]
BackgroundNormal=30,30,30
ForegroundNormal=200,200,200

[Colors:View]
BackgroundNormal=20,20,20
ForegroundNormal=210,210,210

[Colors:Selection]
BackgroundNormal=60,120,200
ForegroundNormal=255,255,255

[Colors:Header]
BackgroundNormal=40,40,40
ForegroundNormal=220,220,220
",
        )
        .unwrap();

        let scheme = parse_color_scheme_file(&path, "TestScheme").unwrap();
        assert_eq!(scheme.name, "Test Scheme");
        assert_eq!(scheme.window_background, Rgb { r: 30, g: 30, b: 30 });
        assert_eq!(scheme.view_foreground, Rgb { r: 210, g: 210, b: 210 });
        assert_eq!(scheme.selection_background, Rgb { r: 60, g: 120, b: 200 });
    }

    #[test]
    fn test_color_scheme_search_paths() {
        let paths = color_scheme_search_paths();
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| p.to_string_lossy().contains("color-schemes")));
    }

    #[test]
    fn test_get_current_color_scheme_no_kde() {
        // In CI without KDE, should return None gracefully
        let result = get_current_color_scheme().unwrap();
        assert!(result.is_none());
    }
}
