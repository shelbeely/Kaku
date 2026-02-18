<div align="center">
  <h1>Kaku</h1>
  <p><em>A fast, KDE Plasma-native terminal built for AI coding on Linux.</em></p>
</div>

<p align="center">
  <a href="https://github.com/tw93/Kaku/stargazers"><img src="https://img.shields.io/github/stars/tw93/Kaku?style=flat-square" alt="Stars"></a>
  <a href="https://github.com/tw93/Kaku/releases"><img src="https://img.shields.io/github/v/tag/tw93/Kaku?label=version&style=flat-square" alt="Version"></a>
  <a href="LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License"></a>
  <a href="https://github.com/tw93/Kaku/commits"><img src="https://img.shields.io/github/commit-activity/m/tw93/Kaku?style=flat-square" alt="Commits"></a>
  <a href="https://twitter.com/HiTw93"><img src="https://img.shields.io/badge/follow-Tw93-red?style=flat-square&logo=Twitter" alt="Twitter"></a>
</p>

<p align="center">
  <img src="assets/kaku.jpeg" alt="Kaku Screenshot" width="1000" />
  <br/>
  Kaku is a deeply customized fork of <a href="https://github.com/wez/wezterm">WezTerm</a>, designed for KDE Plasma and Linux.
</p>

## Features

- **KDE Plasma Native**: Deep integration with KDE desktop environment
- **Zero Config**: Defaults with JetBrains Mono, modern themes, and optimized rendering
- **Built-in Shell Suite**: Pre-loaded Starship, z, Delta, syntax highlighting, autocompletions
- **Fast & Lightweight**: Optimized for Linux, instant startup, GPU-accelerated rendering
- **WezTerm-Compatible Config**: Use WezTerm's Lua config directly with full API compatibility

## Quick Start

### Linux (KDE Plasma)

**📘 See [LINUX.md](LINUX.md) for complete installation guide**  
**🎨 See [KDE_INTEGRATION.md](KDE_INTEGRATION.md) for KDE Plasma integration roadmap**

Kaku is being designed for deep integration with KDE Plasma desktop environment.

**Quick CLI installation** (Ubuntu/KDE Plasma):
```bash
# Install dependencies
sudo apt-get install -y libwayland-dev libxkbcommon-dev libx11-dev libxcb1-dev

# Build and install
cargo build --release -p kaku
sudo cp target/release/kaku /usr/local/bin/
```

**Current Status**:
- ✅ CLI tools fully functional
- 🔧 GUI with full Wayland/X11 support - **IN ACTIVE DEVELOPMENT**
- 🎯 KDE Plasma integration roadmap ready

**Target Environment**: KDE Plasma 6.x on Wayland (primary), Plasma 5.27+ supported

## Usage Guide

Kaku uses intuitive Linux desktop shortcuts:

| Action | Shortcut |
| :--- | :--- |
| New Tab | `Ctrl + Shift + T` |
| New Window | `Ctrl + Shift + N` |
| Close Tab/Pane | `Ctrl + Shift + W` |
| Navigate Tabs | `Ctrl + PgUp`, `Ctrl + PgDn` or `Alt + 1-9` |
| Navigate Panes | `Ctrl + Shift + Arrows` |
| Split Pane Vertical | `Ctrl + Shift + D` |
| Split Pane Horizontal | `Ctrl + Shift + H` |
| Zoom/Unzoom Pane | `Ctrl + Shift + Z` |
| Resize Pane | `Ctrl + Alt + Arrows` |
| Clear Screen | `Ctrl + Shift + K` |
| Font Size | `Ctrl + +`, `Ctrl + -`, `Ctrl + 0` |
| Smart Jump | `z <dir>` |
| Smart Select | `z -l <dir>` |
| Recent Dirs | `z -t` |

## Configuration

Kaku comes with a carefully curated suite of CLI tools, pre-configured for immediate productivity:

- **Starship**: A fast, customizable prompt showing git status, package versions, and execution time
- **z (zoxide)**: A smarter cd command that learns your most used directories
- **Delta**: A syntax-highlighting pager for git, diff, and grep output
- **zsh-completions**: Extended command and subcommand completion definitions
- **Syntax Highlighting**: Real-time command validation and coloring
- **Autosuggestions**: Intelligent, history-based completions

Kaku uses `~/.config/kaku/kaku.lua` for configuration, fully compatible with WezTerm's Lua API.

Run `kaku` in your terminal to see all available commands such as `kaku update`, `kaku reset`, and `kaku config`.

## Why Kaku for Linux/KDE Plasma?

After using various terminal emulators on Linux (Konsole, Alacritty, Kitty), we wanted a terminal that:
- Feels native to KDE Plasma
- Has modern AI coding features built-in
- Provides a great out-of-the-box experience
- Leverages KDE technologies (KWallet, KNotifications, etc.)
- Is fast and GPU-accelerated

WezTerm provides a robust foundation, and Kaku builds on that with practical defaults for Linux/KDE Plasma users, while keeping full Lua-based customization.

## Performance

| Metric | Target |
| :--- | :--- |
| **Executable Size** | ~30-40 MB (optimized for Linux) |
| **Launch Latency** | Instant |
| **Shell Bootstrap** | ~50-100ms |
| **GPU Rendering** | Vulkan on Linux |

Achieved through feature pruning, lazy loading, and Linux-specific optimizations.

## FAQ

1. **Why focus on KDE Plasma?**

   KDE Plasma provides excellent Wayland support, rich DBus APIs for integration, and powerful frameworks (KNotifications, KWallet, KIO). It's the perfect environment for a deeply integrated terminal experience.

2. **Is there a Linux version?**

   **Yes!** You're looking at it. Kaku is now a Linux-first terminal with **primary focus on KDE Plasma** desktop environment.
   
   **Current Status:**
   - ✅ CLI tools - fully functional on Linux
   - 🔧 **GUI with full Wayland/X11 support - IN ACTIVE DEVELOPMENT**
   - 🎯 KDE Plasma integration roadmap ready
   
   **Installation:**
   ```bash
   # Install build dependencies
   sudo apt-get install -y \
       libwayland-dev libxkbcommon-dev \
       libx11-dev libxcb1-dev
   
   # Build
   cargo build --release -p kaku
   ```
   
   **See [LINUX.md](LINUX.md) and [KDE_INTEGRATION.md](KDE_INTEGRATION.md) for details**

3. **What about Windows or macOS?**

   Kaku is now Linux-only, focusing on KDE Plasma integration. For macOS or Windows, use upstream [WezTerm](https://github.com/wez/wezterm).

4. **Which desktop environments are supported?**

   **Primary**: KDE Plasma 6.x on Wayland  
   **Supported**: KDE Plasma 5.27+, other desktop environments (GNOME, XFCE) should work but KDE integration features won't be available.

5. **Can I import my Konsole profiles?**

   Yes! This is planned in our KDE integration roadmap. Konsole profile compatibility is a priority feature.

## Contributors

Big thanks to all contributors who helped build Kaku. Go follow them! ❤️

<a href="https://github.com/tw93/Kaku/graphs/contributors">
  <img src="./CONTRIBUTORS.svg?v=2" width="1000" />
</a>

## Support

- If Kaku helped you, star the repo or [share it](https://twitter.com/intent/tweet?url=https://github.com/tw93/Kaku&text=Kaku%20-%20A%20fast%20terminal%20built%20for%20AI%20coding.) with friends.
- Got ideas or found bugs? Open an issue/PR or check [CONTRIBUTING.md](CONTRIBUTING.md) for details.
- Like Kaku? <a href="https://miaoyan.app/cats.html?name=Kaku" target="_blank">Buy Tw93 a Coke</a> to support the project! 🥤 Supporters below.

<a href="https://miaoyan.app/cats.html?name=Kaku"><img src="https://miaoyan.app/assets/sponsors.svg" width="1000" loading="lazy" /></a>

## License

MIT License, feel free to enjoy and participate in open source.
