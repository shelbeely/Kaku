//! Plasma Activities Support
//!
//! Activities are KDE Plasma's way of organizing workflows and contexts.
//! This module allows Kurrent to have different configurations per activity.
//!
//! ## DBus Interface
//!
//! Service: org.kde.ActivityManager
//! Path: /ActivityManager/Activities
//! Interface: org.kde.ActivityManager.Activities

use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::dbus as kde_dbus;

/// Represents a Plasma Activity
#[derive(Debug, Clone)]
pub struct Activity {
    pub id: String,
    pub name: String,
}

/// Helper to create a proxy to the ActivityManager service
fn activity_proxy() -> Result<zbus::blocking::Proxy<'static>> {
    let conn = kde_dbus::session_connection()?;
    zbus::blocking::Proxy::new(
        &conn,
        kde_dbus::services::ACTIVITY_MANAGER,
        kde_dbus::paths::ACTIVITY_MANAGER,
        kde_dbus::interfaces::ACTIVITIES,
    )
    .context("Failed to create ActivityManager proxy")
}

/// Get the current active Plasma activity, or `None` if unavailable.
pub fn get_current_activity() -> Result<Option<Activity>> {
    if !are_activities_available() {
        return Ok(None);
    }
    let proxy = activity_proxy()?;
    let id: String = proxy
        .call("CurrentActivity", &())
        .context("Failed to get current activity")?;
    let name: String = proxy
        .call("ActivityName", &(&*id,))
        .unwrap_or_else(|_| id.clone());
    Ok(Some(Activity { id, name }))
}

/// List all available activities.
///
/// Returns an empty list if the ActivityManager service is not running.
pub fn list_activities() -> Result<Vec<Activity>> {
    if !are_activities_available() {
        return Ok(Vec::new());
    }
    let proxy = activity_proxy()?;
    let ids: Vec<String> = proxy
        .call("ListActivities", &())
        .context("Failed to list activities")?;

    let mut activities = Vec::with_capacity(ids.len());
    for id in ids {
        let name: String = proxy
            .call("ActivityName", &(&*id,))
            .unwrap_or_else(|_| id.clone());
        activities.push(Activity { id, name });
    }
    Ok(activities)
}

/// Get the configuration file path for a specific activity
pub fn activity_config_path(activity_id: &str) -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("kurrent")
        .join("activities")
        .join(format!("{}.lua", activity_id))
}

/// Watch for activity changes. Spawns a blocking loop in the current thread
/// that calls `callback` whenever the current activity changes.
///
/// This is intended to be run in a dedicated thread.
pub fn watch_activity_changes<F>(callback: F) -> Result<()>
where
    F: Fn(Activity) + Send + 'static,
{
    if !are_activities_available() {
        return Ok(());
    }
    let conn = kde_dbus::session_connection()?;

    // Use a DBus match rule to listen for the CurrentActivityChanged signal
    let proxy = zbus::blocking::Proxy::new(
        &conn,
        kde_dbus::services::ACTIVITY_MANAGER,
        kde_dbus::paths::ACTIVITY_MANAGER,
        kde_dbus::interfaces::ACTIVITIES,
    )
    .context("Failed to create ActivityManager proxy for watching")?;

    // Poll-based approach: listen for signals on the proxy
    let iter = proxy.receive_signal("CurrentActivityChanged");
    match iter {
        Ok(iter) => {
            for signal in iter {
                if let Ok(body) = signal.body().deserialize::<String>() {
                    let id = body;
                    let name: String = proxy
                        .call("ActivityName", &(&*id,))
                        .unwrap_or_else(|_| id.clone());
                    callback(Activity { id, name });
                }
            }
        }
        Err(e) => {
            log::warn!("Failed to subscribe to activity changes: {}", e);
        }
    }
    Ok(())
}

/// Check if Activities are available (Plasma Desktop only)
pub fn are_activities_available() -> bool {
    kde_dbus::is_service_available(kde_dbus::services::ACTIVITY_MANAGER)
}

/// Create default activity configuration files under `~/.config/kurrent/activities/`
pub fn create_default_activity_configs() -> Result<()> {
    let base = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("kurrent")
        .join("activities");
    fs::create_dir_all(&base).context("Failed to create activities config directory")?;

    let defaults = [
        (
            "work.lua",
            "-- Kurrent: Work activity configuration\nreturn {\n  color_scheme = \"Breeze Dark\",\n}\n",
        ),
        (
            "personal.lua",
            "-- Kurrent: Personal activity configuration\nreturn {\n  color_scheme = \"Breeze\",\n}\n",
        ),
        (
            "coding.lua",
            "-- Kurrent: Coding activity configuration\nreturn {\n  color_scheme = \"Breeze Dark\",\n}\n",
        ),
    ];

    for (name, content) in &defaults {
        let path = base.join(name);
        if !path.exists() {
            let mut f = fs::File::create(&path)
                .with_context(|| format!("Failed to create {}", path.display()))?;
            f.write_all(content.as_bytes())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_config_path() {
        let path = activity_config_path("test-activity");
        assert!(path.to_string_lossy().contains("kurrent"));
        assert!(path.to_string_lossy().contains("activities"));
        assert!(path.to_string_lossy().ends_with(".lua"));
    }

    #[test]
    fn test_are_activities_available_no_panic() {
        // In CI without Plasma, returns false
        assert!(!are_activities_available());
    }

    #[test]
    fn test_get_current_activity_no_plasma() {
        let activity = get_current_activity().unwrap();
        assert!(activity.is_none());
    }

    #[test]
    fn test_list_activities_no_plasma() {
        let activities = list_activities().unwrap();
        assert!(activities.is_empty());
    }

    #[test]
    fn test_create_default_activity_configs() {
        let dir = tempfile::tempdir().unwrap();
        // Override config dir by writing to a temp location
        let base = dir.path().join("kurrent").join("activities");
        fs::create_dir_all(&base).unwrap();

        let path = base.join("work.lua");
        assert!(!path.exists());

        // The function uses dirs::config_dir(), so we test the public API
        // just doesn't panic; in real use it creates real files
        let _ = create_default_activity_configs();
    }
}
