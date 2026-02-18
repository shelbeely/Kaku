# Next Session Implementation Guide

## What's Ready

✅ **Renamed**: Kaku → Kurrent  
✅ **Structure Created**: KDE Frameworks crate with stub modules  
✅ **Kirigami Setup**: QML files and C++ bridge skeleton  
✅ **Documentation**: Architecture and integration guides  

## Priority Tasks for Next Session

### 1. Complete GUI Compilation (HIGH PRIORITY)

**Current Status**: ~80% complete, xcb version incompatibility

**Tasks**:
```bash
cd /home/runner/work/Kaku/Kaku

# Check WezTerm's xcb version
cd /tmp/wezterm
grep "^xcb " Cargo.lock

# Update Kurrent's Cargo.toml to match
# Then fix remaining compilation errors

cargo build -p kurrent-gui 2>&1 | tee build.log
# Fix errors iteratively
```

**Expected Issues**:
- xcb API version differences
- Missing trait implementations
- Type inference errors

**Time Estimate**: 2-4 hours

### 2. Implement KDE Color Scheme Detection (MEDIUM PRIORITY)

**File**: `crates/kde-frameworks/src/color_scheme.rs`

**Implementation**:
```rust
use ini::Ini;

pub fn get_current_color_scheme() -> Result<Option<KdeColorScheme>> {
    // 1. Read ~/.config/kdeglobals
    let path = kdeglobals_path();
    let conf = Ini::load_from_file(path)?;
    
    // 2. Get scheme name from [General] section
    let scheme_name = conf
        .section(Some("General"))
        .and_then(|s| s.get("ColorScheme"))?;
    
    // 3. Load the scheme file
    load_color_scheme(scheme_name)
}

pub fn load_color_scheme(name: &str) -> Result<Option<KdeColorScheme>> {
    // Search in ~/.local/share/color-schemes/ and /usr/share/color-schemes/
    // Parse .colors file format
}
```

**Testing**:
```bash
cargo test -p kde-frameworks
# Test on actual KDE Plasma system
```

**Time Estimate**: 2-3 hours

### 3. Implement KNotifications (MEDIUM PRIORITY)

**File**: `crates/kde-frameworks/src/notifications.rs`

**Implementation**:
```rust
use zbus::{Connection, dbus_proxy};

#[dbus_proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: std::collections::HashMap<&str, zbus::zvariant::Value>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

pub fn show_notification(notification: &Notification) -> Result<u32> {
    let conn = Connection::session()?;
    let proxy = NotificationsProxy::new(&conn)?;
    
    let id = proxy.notify(
        "Kurrent",
        0,
        notification.icon.as_deref().unwrap_or("kurrent"),
        &notification.title,
        &notification.body,
        vec![],
        HashMap::new(),
        notification.timeout.unwrap_or(-1),
    )?;
    
    Ok(id)
}
```

**Time Estimate**: 2-3 hours

### 4. Setup Kirigami Build (LOW PRIORITY - Can wait)

**Tasks**:
```bash
cd kurrent-kirigami

# Install dependencies (on Ubuntu/KDE)
sudo apt-get install \
    cmake extra-cmake-modules \
    qt6-base-dev qt6-declarative-dev \
    libkf6kirigami2-dev libkf6i18n-dev \
    libkf6coreaddons-dev libkf6config-dev

# Build
cmake -B build
cmake --build build

# Test
./build/kurrent-kirigami-ui
```

**Time Estimate**: 1-2 hours

### 5. Integrate kde-frameworks into kurrent-gui (MEDIUM PRIORITY)

**File**: `kurrent-gui/Cargo.toml`

Add dependency:
```toml
[dependencies]
kde-frameworks = { path = "../crates/kde-frameworks", optional = true }

[features]
default = ["vendored-fonts", "wayland", "kde-integration"]
kde-integration = ["kde-frameworks"]
```

**File**: `kurrent-gui/src/main.rs` or wherever notifications are used

Replace stub notification with KDE:
```rust
#[cfg(feature = "kde-integration")]
use kde_frameworks::notifications;

fn show_notification(title: &str, message: &str) {
    #[cfg(feature = "kde-integration")]
    {
        let notif = notifications::Notification::new(title, message);
        if let Err(e) = notifications::show_notification(&notif) {
            log::error!("Failed to show KDE notification: {}", e);
        }
    }
    
    #[cfg(not(feature = "kde-integration"))]
    {
        // Fallback
        log::info!("Notification: {} - {}", title, message);
    }
}
```

**Time Estimate**: 1-2 hours

## Task Priority Order

### Session 1 (4-6 hours)
1. ✅ Fix GUI compilation (xcb issues)
2. ✅ Get basic window working on Wayland
3. ✅ Runtime testing and bug fixes

### Session 2 (4-6 hours)
4. Implement color scheme detection
5. Implement KNotifications
6. Test on KDE Plasma 6

### Session 3 (4-6 hours)
7. Implement KWallet for API keys
8. Add Plasma Activities support
9. Integration testing

### Session 4 (Optional - Kirigami)
10. Build Kirigami UI
11. Create Rust FFI interface
12. Embed terminal in Kirigami

## Testing Checklist

### GUI Basic Functionality
- [ ] Window opens on Wayland
- [ ] Window opens on X11
- [ ] Keyboard input works
- [ ] Mouse selection works
- [ ] Can run commands
- [ ] Multiple tabs work

### KDE Integration
- [ ] Detects KDE Plasma environment
- [ ] Reads KDE color scheme
- [ ] Applies colors to terminal
- [ ] Shows native KDE notifications
- [ ] Respects Do Not Disturb mode

### Kirigami (Future)
- [ ] Settings dialog opens
- [ ] Settings can be changed
- [ ] Changes apply to terminal
- [ ] Window chrome looks native

## Quick Commands Reference

```bash
# Build CLI
cargo build --release -p kurrent

# Build GUI
cargo build --release -p kurrent-gui

# Build KDE frameworks
cargo build -p kde-frameworks

# Run tests
cargo test -p kde-frameworks
cargo test -p kurrent-gui

# Build Kirigami UI (when ready)
cd kurrent-kirigami
cmake -B build && cmake --build build
./build/kurrent-kirigami-ui

# Check for macOS references (should be none)
grep -r "macos\|Kaku" --include="*.rs" --include="*.toml" --exclude-dir=target .

# Update documentation
grep -r "Kaku" --include="*.md" .  # Should only be in historical context
```

## Common Issues & Solutions

### Issue: xcb version mismatch
**Solution**: Update `Cargo.toml` to match WezTerm's xcb version

### Issue: "Connection not found"
**Solution**: Check that X11/Wayland modules are properly exported in `window/src/os/mod.rs`

### Issue: KDE detection fails
**Solution**: Check `XDG_CURRENT_DESKTOP` and `KDE_SESSION_VERSION` environment variables

### Issue: DBus connection fails
**Solution**: Ensure `zbus` is properly configured, check session bus is available

## Resources

- **WezTerm upstream**: https://github.com/wez/wezterm
- **KDE DBus docs**: https://techbase.kde.org/Development/Tutorials/D-Bus
- **Kirigami docs**: https://develop.kde.org/frameworks/kirigami/
- **zbus docs**: https://docs.rs/zbus/

## Files to Review Before Starting

1. `window/src/os/x11/connection.rs` - xcb API usage
2. `ARCHITECTURE.md` - Overall system design
3. `KDE_INTEGRATION.md` - KDE feature roadmap
4. `STATUS_REPORT.md` - Current state

## Success Criteria

**Minimum for next milestone**:
- ✅ GUI window opens and works
- ✅ At least one KDE integration working (color scheme OR notifications)
- ✅ Documentation updated

**Ideal**:
- ✅ Full GUI functionality
- ✅ Color scheme + notifications working
- ✅ Kirigami UI built and tested
- ✅ Clear path to Activities and KWallet

Good luck! 🚀
