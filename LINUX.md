# Building Kaku on Linux (Optimized for KDE Plasma)

This guide explains how to build and use Kaku on Linux, with **primary focus on KDE Plasma desktop integration**.

## Vision

Kaku aims to be a first-class citizen in the KDE Plasma ecosystem, deeply integrated with KDE technologies and providing a native KDE experience. See [KDE_INTEGRATION.md](KDE_INTEGRATION.md) for the complete integration roadmap.

## Current Status

- ✅ **CLI Tools**: Fully functional
- ⏳ **GUI**: In progress (requires Wayland/X11 implementation)
- 🎯 **KDE Integration**: Planned - comprehensive Plasma integration roadmap available

The `kaku` command-line tool works great on Linux and can be used to manage terminal configuration. The graphical interface (`kaku-gui`) and KDE Plasma integration features require additional implementation work.

## Prerequisites

### KDE Plasma (Recommended Environment)

**Primary Target**: KDE Plasma 6.x on Wayland  
**Supported**: KDE Plasma 5.27+ on X11 or Wayland

**Recommended Distributions**:
- KDE neon (reference implementation for latest Plasma)
- Kubuntu 24.04+
- Fedora KDE Spin
- openSUSE Tumbleweed KDE

### Ubuntu 22.04+ / KDE Plasma (any version)

Kaku is tested on Ubuntu 22.04 and later. KDE Plasma 5.x and 6.x are both supported.

Install the required development libraries:

```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libwayland-dev \
    libxkbcommon-dev \
    libx11-dev \
    libxcb1-dev \
    libxcb-util-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxcb-keysyms1-dev \
    libxcb-image0-dev \
    libfontconfig1-dev \
    libfreetype-dev
```

### Rust Toolchain

Install Rust if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Building

### Build the CLI Tool

```bash
# Clone the repository
git clone https://github.com/tw93/Kaku.git
cd Kaku

# Build the kaku CLI
cargo build --release -p kaku

# The binary will be at:
# target/release/kaku
```

### Install Locally

```bash
# Copy to a directory in your PATH
sudo cp target/release/kaku /usr/local/bin/

# Verify installation
kaku --version
```

## Usage on Linux

The `kaku` CLI tool provides several useful commands:

```bash
# Show available commands
kaku --help

# Example commands
kaku config     # Open configuration
kaku reset      # Reset settings
kaku update     # Check for updates
```

## KDE Plasma Integration

While the GUI is not yet available, Kaku is being designed for deep KDE Plasma integration:

### Planned KDE Features

**🎨 Visual Integration**
- Auto-detect and apply KDE Plasma color schemes
- Sync with Plasma dark/light theme switching
- Use KDE window decorations and effects
- Respect Plasma accent colors

**🔔 Desktop Integration**
- KNotifications for native KDE notifications
- System tray integration
- Plasma Activities support (different configs per activity)
- KRunner plugin for quick launch

**🔐 KDE Frameworks**
- KWallet integration for secure credential storage (AI API keys)
- KIO file picker dialogs
- Konsole profile compatibility and import
- KDE global shortcuts integration

**📋 See [KDE_INTEGRATION.md](KDE_INTEGRATION.md) for the complete integration roadmap**

### Current KDE Compatibility

The configuration system is compatible with WezTerm's Lua API and ready for KDE-specific extensions:

```lua
-- Future: ~/.config/kaku/kde.lua
local config = {}

-- Auto-theme from KDE Plasma (planned)
config.kde_auto_theme = true
config.kde_sync_appearance = true
config.kde_use_accent_color = true

-- Activities support (planned)
config.kde_activity_profiles = {
  work = "~/.config/kaku/activities/work.lua",
  personal = "~/.config/kaku/activities/personal.lua",
}

return config
```

## Configuration

Kaku uses `~/.config/kaku/kaku.lua` for configuration, which is compatible with WezTerm's Lua API:

```lua
-- Example ~/.config/kaku/kaku.lua
local config = {}

config.font_size = 11.0
config.color_scheme = "One Dark"
config.enable_tab_bar = true

return config
```

## Next Steps for Complete Linux Support

Kaku is a fork of [WezTerm](https://github.com/wez/wezterm), which has full Linux support including:
- Complete Wayland implementation
- X11 fallback support
- Multiple desktop environment integrations

To achieve full GUI support on Linux, the upstream WezTerm's Linux window implementation could be adapted. Key areas to port:

1. **Window Implementation** - From WezTerm's `window` crate
   - Wayland protocol implementation
   - X11 window management
   - Event loop integration

2. **Desktop Integration**
   - KDE Plasma theme detection (WezTerm has this)
   - Native notification system (libnotify)
   - Desktop entry files

3. **Shell Integration**
   - Linux-specific shell setup scripts
   - Terminal emulator detection
   - Path management

**For contributors**: See the [upstream WezTerm repository](https://github.com/wez/wezterm) for reference implementations of all Linux windowing features.

## Known Limitations

1. **GUI Not Available**: The graphical terminal window (`kaku-gui`) doesn't work yet on Linux
2. **Wayland/X11 Support**: Window management integration is still being implemented
3. **Shell Integration**: Some macOS-specific shell features may not work on Linux

## Contributing

We welcome contributions to improve Linux support! 

**Recommended Approach**: Kaku is a fork of [WezTerm](https://github.com/wez/wezterm), which already has complete Linux support. The most effective path forward is to adapt WezTerm's Linux implementation to Kaku.

### Priority Areas

1. **Port WezTerm's Wayland Implementation**
   - Reference: `wezterm/window/src/os/wayland/` in upstream
   - Wayland window protocol
   - Compositor integration

2. **Port WezTerm's X11 Support**
   - Reference: `wezterm/window/src/os/x11/` in upstream
   - X11 window management
   - XCB event handling

3. **Desktop Integration**
   - KDE Plasma theme integration (WezTerm has this working)
   - Native notifications (libnotify)
   - Desktop entry files

4. **Shell Integration**
   - Adapt Linux-specific shell scripts from WezTerm

### Getting Started

1. Clone both repositories:
   ```bash
   git clone https://github.com/tw93/Kaku.git
   git clone https://github.com/wez/wezterm.git
   ```

2. Compare the `window` crate structures:
   - Kaku: `window/src/os/` (currently has macOS + Linux stub)
   - WezTerm: `window/src/os/` (has macOS + Wayland + X11)

3. Port the Wayland/X11 modules while maintaining Kaku's customizations

For questions or guidance, open an issue in the Kaku repository.

## Troubleshooting

### Build Fails with Missing Library

Make sure all development libraries are installed:
```bash
sudo apt-get install -y libwayland-dev libxkbcommon-dev libx11-dev libxcb1-dev
```

### Permission Denied

The `kaku` binary needs execute permissions:
```bash
chmod +x target/release/kaku
```

## Support

For Linux-specific issues, please file a bug report at:
https://github.com/tw93/Kaku/issues

Include:
- Your Ubuntu/Linux version
- Desktop environment (KDE Plasma version)
- Error messages
- Output of `kaku --version`
