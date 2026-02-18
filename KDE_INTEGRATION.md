# KDE Plasma Integration for Kurrent

This document outlines the deep integration of Kurrent with the KDE Plasma desktop environment and KDE ecosystem.

## KDE HIG Compliance

Kurrent follows the [KDE Human Interface Guidelines](https://develop.kde.org/hig/) to be a first-class KDE application:

| Requirement | Status | Details |
|---|---|---|
| XDG Desktop Entry (`.desktop`) | ✅ | `org.kde.kurrent.desktop` with actions, categories, keywords |
| AppStream MetaInfo | ✅ | `org.kde.kurrent.metainfo.xml` for KDE Discover |
| SVG application icon | ✅ | `org.kde.kurrent.svg` installed via ECMInstallIcons |
| KAboutData in `main.cpp` | ✅ | Standard KDE `--version` / `--author` CLI, about dialog |
| KLocalizedString / i18n | ✅ | All user-visible QML strings wrapped in `i18n()` |
| Accessibility annotations | ✅ | `Accessible.name` / `.description` on all interactive elements |
| KDE ECM CMake integration | ✅ | KDEInstallDirs, KDECMakeSettings, ECMInstallIcons |
| Reverse-domain naming | ✅ | `org.kde.kurrent` everywhere |
| KDE color scheme following | ✅ | `kde-frameworks` crate reads `kdeglobals` |
| KWallet credential storage | ✅ | `kde-frameworks` crate wraps KWallet V5/V6 |
| freedesktop Notifications | ✅ | `kde-frameworks` crate sends via DBus |
| Plasma Activities support | ✅ | `kde-frameworks` crate queries ActivityManager |

## Vision

Make Kaku a first-class citizen in the KDE Plasma ecosystem, feeling native and deeply integrated with KDE technologies, frameworks, and user experience patterns.

## KDE Integration Roadmap

### Phase 1: Core Desktop Integration (Priority)

#### 1.1 KDE Color Schemes & Theming
- **Goal**: Automatically detect and apply KDE Plasma color schemes
- **Implementation**:
  - Read KDE color scheme from `~/.config/kdeglobals`
  - Parse `[Colors:*]` sections for window, view, selection colors
  - Auto-apply matching terminal color scheme
  - Support Plasma dark/light theme switching via DBus signals
- **KDE Technologies**: `kdeglobals`, DBus color scheme service
- **Files to Create**: `crates/kde-integration/src/color_scheme.rs`

#### 1.2 KNotifications Integration
- **Goal**: Use KDE's notification system instead of custom toasts
- **Implementation**:
  - Replace stub in `crates/wezterm-toast-notification/src/linux.rs`
  - Use KNotifications5/6 library via DBus
  - Support notification actions (click to focus, close, etc.)
  - Respect KDE notification settings and Do Not Disturb mode
- **Dependencies**: `knotifications` DBus interface
- **Files to Modify**: `crates/wezterm-toast-notification/src/linux.rs`

#### 1.3 KDE Window Decorations
- **Goal**: Use Plasma window decorations and respect KDE window settings
- **Implementation**:
  - Support client-side decorations (CSD) with Plasma style
  - Respect KDE's titlebar button layout preferences
  - Support Plasma-specific window rules
  - Integrate with KWin effects (blur, transparency, shadows)
- **KDE Technologies**: KWin, Plasma window decoration engine
- **Files to Create**: `window/src/os/wayland/kde_decoration.rs`

#### 1.4 DBus Integration
- **Goal**: Communicate with KDE services via DBus
- **Implementation**:
  - Listen for Plasma theme changes
  - Integrate with KDE session management
  - Support Plasma activities
  - Respond to global shortcuts via KGlobalAccel
- **Dependencies**: `zbus` (already in workspace dependencies)
- **Files to Create**: `crates/kde-integration/src/dbus.rs`

### Phase 2: Advanced KDE Features

#### 2.1 KWallet Integration
- **Goal**: Store sensitive data (API keys, credentials) in KWallet
- **Implementation**:
  - Store AI coding assistant API keys securely
  - Integrate with KWallet DBus API
  - Support both KWallet5 and KWallet6
  - Auto-unlock integration with KDE session
- **Dependencies**: KWallet DBus interface
- **Files to Create**: `crates/kde-integration/src/kwallet.rs`

#### 2.2 Plasma Activities Support
- **Goal**: Support KDE Activities for different work contexts
- **Implementation**:
  - Separate configurations per activity
  - Auto-switch profiles based on active activity
  - Activity-specific shell environments
- **Dependencies**: KDE Activities DBus API
- **Files to Create**: `crates/kde-integration/src/activities.rs`

#### 2.3 KIO Integration
- **Goal**: Use KDE's file operations framework
- **Implementation**:
  - Use KDE file picker dialogs
  - Support KIO protocols (sftp://, fish://, etc.) in file operations
  - Integration with Dolphin file manager
- **Dependencies**: KIO framework
- **Files to Create**: `crates/kde-integration/src/kio.rs`

#### 2.4 Konsole Compatibility
- **Goal**: Be compatible with Konsole profiles and color schemes
- **Implementation**:
  - Import Konsole color schemes
  - Support Konsole keybindings
  - Compatible with Konsole profiles format
  - Option to migrate from Konsole
- **Files to Create**: `crates/kde-integration/src/konsole_compat.rs`

### Phase 3: Plasma-Specific Features

#### 3.1 Global Menu Support
- **Goal**: Integrate with Plasma's global menu bar
- **Implementation**:
  - Export menu structure via DBus
  - Support application menu protocol
  - Respect user's global menu preferences
- **KDE Technologies**: `com.canonical.AppMenu.Registrar`

#### 3.2 Plasma Widgets Integration
- **Goal**: Provide Plasma widgets/applets for Kaku
- **Implementation**:
  - Quick terminal drop-down widget
  - Terminal session manager applet
  - System tray integration with running sessions
- **Dependencies**: Plasma framework, QML
- **Future Work**: Requires QML/Plasma widget development

#### 3.3 KRunner Integration
- **Goal**: Launch Kaku sessions from KRunner
- **Implementation**:
  - KRunner plugin for quick terminal launch
  - Search and open recent sessions
  - Quick commands execution
- **Dependencies**: KRunner DBus API

#### 3.4 Plasma Vaults Support
- **Goal**: Support encrypted folders via Plasma Vaults
- **Implementation**:
  - Detect when working directory is in a vault
  - Auto-mount integration
  - Vault-specific security settings

### Phase 4: KDE Frameworks Usage

#### 4.1 Recommended KDE Frameworks to Use

```toml
# Potential dependencies for full KDE integration
# (to be added to Cargo.toml when implementing)

[target.'cfg(all(unix, not(target_os="macos")))'.dependencies]
# For KDE integration (via DBus for now, native bindings future work)
zbus = { version = "4.2", features = ["blocking-api"] }  # Already in workspace

# Future: Consider KDE Frameworks Rust bindings when mature
# - kconfig: KDE configuration system
# - knotifications: Native KDE notifications  
# - kio: File operations and protocols
# - kwallet: Secure credential storage
```

#### 4.2 KDE Configuration System
- Use KConfig to store Kaku preferences
- Support KDE's configuration backup/restore
- Integrate with System Settings where appropriate

## Implementation Priority

### High Priority (Phase 1)
1. ✅ Basic Wayland/X11 window support
2. KDE color scheme detection and auto-theming
3. KNotifications for desktop notifications
4. KDE window decorations integration
5. Basic DBus communication with Plasma

### Medium Priority (Phase 2)
6. KWallet integration for API key storage
7. Plasma Activities support
8. Konsole profile compatibility
9. KIO file picker integration

### Low Priority (Phase 3-4)
10. Global menu support
11. Plasma widgets/applets
12. KRunner plugin
13. Plasma Vaults integration

## Technical Architecture

### KDE Integration Crate Structure

```
crates/kde-integration/
├── Cargo.toml
├── src/
│   ├── lib.rs                  # Main KDE integration module
│   ├── color_scheme.rs         # Plasma color scheme detection
│   ├── notifications.rs        # KNotifications integration
│   ├── dbus.rs                 # DBus service communication
│   ├── kwallet.rs             # KWallet integration
│   ├── activities.rs          # Plasma Activities support
│   ├── kio.rs                 # KIO file operations
│   ├── konsole_compat.rs      # Konsole compatibility
│   └── window_integration.rs  # KWin/Plasma window features
```

### Configuration Hierarchy

```
~/.config/kaku/
├── kaku.lua                    # Main config (WezTerm compatible)
├── kde.lua                     # KDE-specific overrides
├── activities/                 # Per-activity configs
│   ├── work.lua
│   ├── personal.lua
│   └── coding.lua
└── konsole-import/            # Imported Konsole profiles
    ├── profiles/
    └── colorschemes/
```

## KDE-Specific Features to Implement

### Auto-Theming
```lua
-- Example ~/.config/kaku/kde.lua
local config = {}

-- Auto-detect and apply KDE Plasma color scheme
config.kde_auto_theme = true

-- Sync with Plasma dark/light mode
config.kde_sync_appearance = true

-- Use Plasma accent color for highlights
config.kde_use_accent_color = true

return config
```

### KDE Shortcuts Integration
```lua
-- Respect KDE global shortcuts
config.kde_global_shortcuts = {
  -- Use KGlobalAccel for system-wide terminal drop-down
  drop_down = "Meta+`",
  
  -- Integrate with KDE's shortcut system
  respect_kde_shortcuts = true,
}
```

## Testing on KDE Plasma

### Target Environments
- **Primary**: KDE Plasma 6.x on Wayland (latest)
- **Secondary**: KDE Plasma 5.27+ on X11
- **Distributions**: 
  - KDE neon (reference implementation)
  - Kubuntu 24.04+
  - Fedora KDE Spin
  - openSUSE Tumbleweed KDE

### Testing Checklist
- [ ] Color scheme auto-detection works
- [ ] Dark/light theme switching is seamless
- [ ] Notifications use KDE style and respect settings
- [ ] Window decorations match Plasma theme
- [ ] Activities switching works correctly
- [ ] KWallet integration is secure
- [ ] Global menu appears correctly
- [ ] System tray integration works
- [ ] Konsole profiles import successfully
- [ ] KRunner plugin launches Kaku

## Resources & References

### KDE Documentation
- [KDE Frameworks API Documentation](https://api.kde.org/frameworks/)
- [Plasma Development](https://develop.kde.org/docs/plasma/)
- [KNotifications](https://api.kde.org/frameworks/knotifications/html/)
- [KWallet](https://api.kde.org/frameworks/kwallet/html/)
- [KIO](https://api.kde.org/frameworks/kio/html/)

### DBus APIs
- [KDE DBus Interfaces](https://techbase.kde.org/Development/Tutorials/D-Bus)
- [org.kde.KWin](https://invent.kde.org/plasma/kwin/-/tree/master/src/dbus)
- [org.kde.StatusNotifier](https://www.freedesktop.org/wiki/Specifications/StatusNotifierItem/)

### Color Scheme Format
- `~/.config/kdeglobals` - KDE global configuration
- `~/.local/share/color-schemes/` - User color schemes
- `/usr/share/color-schemes/` - System color schemes

### Example Code
```rust
// Example: Reading KDE color scheme
use ini::Ini;

pub fn get_kde_colors() -> Result<ColorScheme> {
    let config_path = dirs::home_dir()
        .unwrap()
        .join(".config/kdeglobals");
    
    let conf = Ini::load_from_file(config_path)?;
    
    let window_bg = conf
        .section(Some("Colors:Window"))
        .and_then(|s| s.get("BackgroundNormal"));
    
    // Parse and return colors...
}
```

## Community & Contributions

We especially welcome contributions from KDE developers and users! 

**Join the conversation**:
- KDE development forums
- Plasma development mailing list
- #kde-devel on Matrix/IRC

**Help wanted**:
- KDE developers familiar with KNotifications, KWallet, KIO
- Plasma themers and color scheme creators
- Wayland/KWin integration experts
- QML developers for potential Plasma widgets

## Future Vision

Kaku should become:
- The recommended terminal for KDE Plasma
- Deeply integrated into KDE Plasma workflow
- A showcase for KDE Frameworks capabilities
- A bridge between AI coding tools and KDE desktop
- Feel as native as Konsole, but with modern AI features
