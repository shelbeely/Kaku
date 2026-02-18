<div align="center">
  <h1>Kaku</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
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
  Kaku is a deeply customized fork of <a href="https://github.com/wez/wezterm">WezTerm</a>, designed for an out-of-the-box experience.
</p>

## Features

- **Zero Config**: Defaults with JetBrains Mono, opencode theme, macOS font rendering, and low-res font sizing.
- **Built-in Shell Suite**: Pre-loaded Starship, z, Delta, syntax highlighting, autocompletions, and zsh history.
- **Fast & Lightweight**: 40% smaller binary, instant startup, lazy loading, stripped-down GPU-accelerated core.
- **WezTerm-Compatible Config**: Use WezTerm's Lua config directly with full API compatibility and no migration.

## Quick Start

### macOS

1. [Download Kaku DMG](https://github.com/tw93/Kaku/releases/latest) & Drag to Applications
2. Or install with Homebrew: `brew install tw93/tap/kakuku`
3. Open Kaku. The app is notarized by Apple, so it opens without security warnings
4. On first launch, Kaku will automatically set up your shell environment

### Linux (Ubuntu/KDE Plasma)

**📘 See [LINUX.md](LINUX.md) for complete Linux installation and usage guide.**

Quick CLI installation:
```bash
# Install dependencies
sudo apt-get install -y libwayland-dev libxkbcommon-dev libx11-dev libxcb1-dev

# Build and install
cargo build --release -p kaku
sudo cp target/release/kaku /usr/local/bin/
```

**Note**: GUI support is currently in development. CLI tools are fully functional.

## Usage Guide

Kaku comes with intuitive macOS-native shortcuts:

| Action | Shortcut |
| :--- | :--- |
| New Tab | `Cmd + T` |
| New Window | `Cmd + N` |
| Close Tab/Pane | `Cmd + W` |
| Navigate Tabs | `Cmd + Shift + [`, `Cmd + Shift + ]` or `Cmd + 1-9` |
| Navigate Panes | `Cmd + Opt + Arrows` |
| Split Pane Vertical | `Cmd + D` |
| Split Pane Horizontal | `Cmd + Shift + D` |
| Toggle Split Direction | `Cmd + Shift + S` |
| Zoom/Unzoom Pane | `Cmd + Shift + Enter` |
| Resize Pane | `Cmd + Ctrl + Arrows` |
| Clear Screen | `Cmd + K` |
| Font Size | `Cmd + +`, `Cmd + -`, `Cmd + 0` |
| Smart Jump | `z <dir>` |
| Smart Select | `z -l <dir>` |
| Recent Dirs | `z -t` |

## Configuration

Kaku comes with a carefully curated suite of CLI tools, pre-configured for immediate productivity:

- **Starship**: A fast, customizable prompt showing git status, package versions, and execution time.
- **z**: A smarter cd command that learns your most used directories for instant navigation.
- **Delta**: A syntax-highlighting pager for git, diff, and grep output.
- **zsh-completions**: Extended command and subcommand completion definitions.
- **Syntax Highlighting**: Real-time command validation and coloring.
- **Autosuggestions**: Intelligent, history-based completions similar to Fish shell.

Kaku uses `~/.config/kaku/kaku.lua` for configuration, fully compatible with WezTerm's Lua API, with built-in defaults at `Kaku.app/Contents/Resources/kaku.lua` as fallback.

Run `kaku` in your terminal to see all available commands such as `kaku update`, `kaku reset`, and `kaku config`.

## Why Kaku?

I heavily rely on the CLI for both work and personal projects. Tools I've built, like [Mole](https://github.com/tw93/mole) and [Pake](https://github.com/tw93/pake), reflect this.

I used Alacritty for years and learned to value speed and simplicity. As my workflow shifted toward AI-assisted coding, I wanted stronger tab and pane ergonomics. I also explored Kitty, Ghostty, Warp, and iTerm2. Each is strong in different areas, but I still wanted a setup that matched my own balance of performance, defaults, and control.

WezTerm is robust and highly hackable, and I am grateful for its engine and ecosystem. Kaku builds on that foundation with practical defaults for day one use, while keeping full Lua-based customization and a fast, lightweight feel.

So I built Kaku to be that environment: fast, polished, and ready to work.

### Performance

| Metric | Upstream | Kaku | Methodology |
| :--- | :--- | :--- | :--- |
| **Executable Size** | ~67 MB | ~40 MB | Aggressive symbol stripping & feature pruning |
| **Resources Volume** | ~100 MB | ~80 MB | Asset optimization & lazy-loaded assets |
| **Launch Latency** | Standard | Instant | Just-in-time initialization |
| **Shell Bootstrap** | ~200ms | ~100ms | Optimized environment provisioning |

Achieved through aggressive stripping of unused features, lazy loading of color schemes, and shell optimizations.

## FAQ

1. **Why is the Homebrew cask named `kakuku` instead of `kaku`?**

   The name `kaku` conflicts with another package in Homebrew's official repository (an unmaintained music player). `kakuku` is a cute variation that's easy to remember.

2. **Is there a Linux version?**

   **Yes!** Linux support is now available for Ubuntu and KDE Plasma-based distributions. Currently, the CLI tools (`kaku` command) are fully functional on Linux. The GUI (`kaku-gui`) requires additional implementation work for Wayland/X11 support.
   
   **What works on Linux:**
   - ✅ The `kaku` CLI binary for terminal configuration
   - ✅ All command-line utilities and shell integration tools
   - ⏳ GUI support is in progress (contributions welcome!)
   
   **Ubuntu/KDE Plasma Installation:**
   ```bash
   # Install build dependencies
   sudo apt-get install -y \
       libwayland-dev libxkbcommon-dev \
       libx11-dev libxcb1-dev \
       libxcb-util-dev libxcb-render0-dev \
       libxcb-shape0-dev libxcb-xfixes0-dev \
       libxcb-keysyms1-dev libxcb-image0-dev \
       libfontconfig1-dev libfreetype-dev
   
   # Build Kaku
   cargo build --release -p kaku
   
   # The binary will be at target/release/kaku
   ```

3. **Is there a Windows version?**

   Not at the moment. Windows support may come later once the macOS and Linux versions are mature.

4. **Can Kaku use transparent windows on macOS?**

   Yes. You can set `window_background_opacity` and optionally `macos_window_background_blur` in `~/.config/kaku/kaku.lua`. Transparent mode now keeps top/right/bottom padding regions visually consistent to avoid transparent gaps.

5. **Why does Kaku fail to start in some virtual macOS environments?**

   This usually means the VM has no usable GPU backend (`failed to create NSOpenGLPixelFormat`).
   Enable VM GPU acceleration, or set `config.front_end = 'WebGpu'` in `~/.config/kaku/kaku.lua`.

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
