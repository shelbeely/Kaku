//! Plasma Activities Support
//!
//! Activities are KDE Plasma's way of organizing workflows and contexts.
//! This module allows Kurrent to have different configurations per activity.
//!
//! ## Use Cases
//!
//! - Work activity: Corporate color scheme, specific shell environment
//! - Personal activity: Different theme, personal shortcuts
//! - Coding activity: AI assistant enabled, specific project paths
//!
//! ## Implementation Plan
//!
//! 1. Detect current Plasma activity via DBus
//! 2. Load activity-specific config from ~/.config/kurrent/activities/
//! 3. Listen for activity switches
//! 4. Reload configuration when activity changes
//!
//! ## DBus Interface
//!
//! Service: org.kde.ActivityManager
//! Path: /ActivityManager/Activities
//! Interface: org.kde.ActivityManager.Activities
//!
//! Methods:
//! - CurrentActivity() -> string (activity ID)
//! - ActivityName(id) -> string
//! - ListActivities() -> string[]
//!
//! Signals:
//! - CurrentActivityChanged(id)

use anyhow::Result;
use std::path::PathBuf;

/// Represents a Plasma Activity
#[derive(Debug, Clone)]
pub struct Activity {
    pub id: String,
    pub name: String,
}

/// Get the current active Plasma activity
pub fn get_current_activity() -> Result<Option<Activity>> {
    // TODO: Query via DBus
    // org.kde.ActivityManager.Activities.CurrentActivity()
    log::info!("Would query current Plasma activity");
    Ok(None)
}

/// List all available activities
pub fn list_activities() -> Result<Vec<Activity>> {
    // TODO: Query via DBus
    // org.kde.ActivityManager.Activities.ListActivities()
    Ok(Vec::new())
}

/// Get the configuration file path for a specific activity
pub fn activity_config_path(activity_id: &str) -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("kurrent")
        .join("activities")
        .join(format!("{}.lua", activity_id))
}

/// Watch for activity changes and call callback when activity switches
pub fn watch_activity_changes<F>(callback: F) -> Result<()>
where
    F: Fn(Activity) + Send + 'static,
{
    // TODO: Connect to DBus and listen for CurrentActivityChanged signal
    // When signal received, call callback with new activity
    Ok(())
}

/// Check if Activities are available (Plasma Desktop only)
pub fn are_activities_available() -> bool {
    // TODO: Check if org.kde.ActivityManager service is available on DBus
    false
}

/// Create default activity configurations
pub fn create_default_activity_configs() -> Result<()> {
    // TODO: Create example configs in ~/.config/kurrent/activities/
    // - work.lua
    // - personal.lua
    // - coding.lua
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
}
