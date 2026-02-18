# Building Kaku on Linux (Ubuntu/KDE Plasma)

This guide explains how to build and use Kaku on Ubuntu and KDE Plasma-based Linux distributions.

## Current Status

- ✅ **CLI Tools**: Fully functional
- ⏳ **GUI**: In progress (contributions welcome!)

The `kaku` command-line tool works great on Linux and can be used to manage terminal configuration, but the graphical interface (`kaku-gui`) requires additional Wayland/X11 implementation work.

## Prerequisites

### Ubuntu 22.04+ / KDE Plasma

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

While the GUI is not yet available, you can use Kaku's CLI tools alongside your existing terminal emulator of choice on KDE Plasma (such as Konsole or Alacritty).

The configuration system is compatible, so when GUI support is added, your existing configuration will work seamlessly.

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

## Known Limitations

1. **GUI Not Available**: The graphical terminal window (`kaku-gui`) doesn't work yet on Linux
2. **Wayland/X11 Support**: Window management integration is still being implemented
3. **Shell Integration**: Some macOS-specific shell features may not work on Linux

## Contributing

We welcome contributions to improve Linux support! Priority areas:

- Wayland window implementation
- X11 window implementation  
- KDE Plasma desktop integration
- Linux-specific shell integration scripts

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
