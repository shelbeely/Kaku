# Full Linux GUI Implementation Plan

## Status: READY TO IMPLEMENT

User requirement: **Full GUI support for Linux/KDE Plasma**

## Implementation Strategy

### Approach: Port from Upstream WezTerm

WezTerm (our upstream) has complete, production-ready Linux GUI support:
- **Wayland**: Primary protocol, full implementation
- **X11**: Fallback, complete implementation  
- **Shared**: Common code in `x_and_wayland.rs`

**Source**: https://github.com/wez/wezterm (window/src/os/)

### Phase 1: Foundation (IMMEDIATE - Start Now)

#### Step 1: Copy Core Files from Upstream
```bash
# From wezterm/window/src/os/ to kaku/window/src/os/
├── wayland/           # Wayland implementation (~15 files)
├── x11/              # X11 implementation (~10 files)  
├── x_and_wayland.rs  # Shared code
├── xdg_desktop_portal.rs
└── xkeysyms.rs
```

#### Step 2: Update mod.rs
Replace stub in `window/src/os/linux/mod.rs` with proper Wayland/X11 selection logic from upstream.

#### Step 3: Dependencies Check
Ensure all required dependencies are in Cargo.toml:
- ✅ wayland-client, wayland-protocols (already added)
- ✅ x11, xcb, xcb-imdkit (already added)
- ✅ smithay-client-toolkit (already added)
- ✅ xkbcommon (already added)

#### Step 4: Adapt to Kaku
- Keep Kaku-specific customizations
- Maintain simplified configuration approach
- Preserve Kaku's reduced binary size optimizations

### Phase 2: KDE Plasma Integration (AFTER Phase 1 works)

Once basic GUI works, add KDE-specific features:
1. KDE color scheme detection
2. KNotifications
3. KWallet integration
4. Plasma Activities support

### Phase 3: Testing & Polish

Test on:
- KDE Plasma 6.x on Wayland
- KDE Plasma 5.27 on X11
- Other desktop environments (GNOME, etc.)

## File-by-File Port Guide

### Priority 1: Core Wayland (Start Here)

```
From wezterm/window/src/os/wayland/:
1. mod.rs           - Main Wayland module
2. connection.rs    - Wayland connection
3. window.rs        - Wayland window implementation
4. frame.rs         - Window frame/decorations
5. pointer.rs       - Mouse/pointer handling
6. keyboard.rs      - Keyboard input
7. surface.rs       - Surface management
```

### Priority 2: Core X11 (Fallback)

```
From wezterm/window/src/os/x11/:
1. mod.rs          - Main X11 module
2. connection.rs   - X11 connection  
3. window.rs       - X11 window implementation
4. keyboard.rs     - Keyboard input
5. xsettings.rs    - X settings (DPI, themes)
```

### Priority 3: Shared Code

```
From wezterm/window/src/os/:
1. x_and_wayland.rs      - Common Linux code
2. xdg_desktop_portal.rs - Desktop portal integration
3. xkeysyms.rs           - X key symbol mappings
```

## Execution Steps

### Step-by-Step Implementation

1. **Copy Wayland Implementation**
   ```bash
   cp -r /tmp/wezterm/window/src/os/wayland window/src/os/
   ```

2. **Copy X11 Implementation**
   ```bash
   cp -r /tmp/wezterm/window/src/os/x11 window/src/os/
   ```

3. **Copy Shared Files**
   ```bash
   cp /tmp/wezterm/window/src/os/x_and_wayland.rs window/src/os/
   cp /tmp/wezterm/window/src/os/xdg_desktop_portal.rs window/src/os/
   cp /tmp/wezterm/window/src/os/xkeysyms.rs window/src/os/
   ```

4. **Update os/mod.rs**
   Replace stub with proper platform selection:
   ```rust
   #[cfg(all(unix, not(target_os = "macos")))]
   pub mod x_and_wayland;
   
   #[cfg(all(unix, not(target_os = "macos"), feature = "wayland"))]
   pub mod wayland;
   
   #[cfg(all(unix, not(target_os = "macos")))]
   pub mod x11;
   
   // Selection logic from upstream
   ```

5. **Fix Compilation Issues**
   - Update paths/imports for Kaku's structure
   - Adapt any Kaku-specific APIs
   - Ensure compatibility with Kaku's config system

6. **Test Basic Window**
   ```bash
   cargo build -p kaku-gui
   ./target/debug/kaku-gui
   ```

7. **Iterate Until Working**
   - Fix compile errors
   - Test on Wayland first
   - Test X11 fallback
   - Verify input handling

## Time Estimate

- **Phase 1 (Basic GUI)**: 4-8 hours of focused work
  - File copying: 30 min
  - Compilation fixes: 2-4 hours
  - Basic testing: 1-2 hours
  - Bug fixes: 1-2 hours

- **Phase 2 (KDE Integration)**: 8-16 hours
  - Color scheme: 2-3 hours
  - KNotifications: 2-3 hours
  - Other features: 4-10 hours

- **Phase 3 (Polish)**: 4-8 hours
  - Testing: 2-3 hours
  - Bug fixes: 2-5 hours

**Total Estimate**: 16-32 hours

## Risk Mitigation

### Potential Issues

1. **API Differences**: WezTerm vs Kaku internal APIs
   - **Solution**: Adapt incrementally, keep changes minimal

2. **Dependency Versions**: May differ between WezTerm and Kaku
   - **Solution**: Check Cargo.toml, update if needed

3. **Kaku-Specific Features**: May conflict with ported code
   - **Solution**: Identify conflicts early, adapt carefully

## Success Criteria

### Minimum Viable GUI
- [ ] Window opens on Wayland
- [ ] Window opens on X11 (fallback)
- [ ] Keyboard input works
- [ ] Mouse input works
- [ ] Basic rendering works
- [ ] Can run shell commands

### KDE Integration
- [ ] Detects KDE Plasma
- [ ] Applies KDE color scheme
- [ ] Shows KDE notifications
- [ ] Window decorations match KDE theme

## Next Actions

**IMMEDIATE**: Start Phase 1, Step 1
```bash
# Execute this to begin:
cd /home/runner/work/Kaku/Kaku
cp -r /tmp/wezterm/window/src/os/wayland window/src/os/
cp -r /tmp/wezterm/window/src/os/x11 window/src/os/
cp /tmp/wezterm/window/src/os/x_and_wayland.rs window/src/os/
cp /tmp/wezterm/window/src/os/xdg_desktop_portal.rs window/src/os/
cp /tmp/wezterm/window/src/os/xkeysyms.rs window/src/os/
```

Then update `window/src/os/mod.rs` with proper module declarations.
