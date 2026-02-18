# Kurrent Architecture

## Overview

Kurrent uses a **hybrid architecture** combining Rust for high-performance terminal rendering with Qt/QML (Kirigami) for native KDE Plasma UI integration.

```
┌─────────────────────────────────────────────┐
│         Kirigami UI Layer (Qt/QML)          │
│  ┌───────────┬──────────────┬─────────┐    │
│  │  Window   │   Settings   │  Menus  │    │
│  │  Chrome   │   Dialog     │         │    │
│  └─────┬─────┴──────┬───────┴─────┬───┘    │
│        │            │             │         │
│        └────────────┼─────────────┘         │
│                     │                        │
│            ┌────────▼──────────┐            │
│            │  Terminal Bridge  │            │
│            │   (Qt ↔ Rust)     │            │
│            └────────┬──────────┘            │
└─────────────────────┼───────────────────────┘
                      │ FFI
┌─────────────────────▼───────────────────────┐
│      Rust Terminal Core (kurrent-gui)       │
│  ┌──────────┬────────────┬──────────────┐  │
│  │ Terminal │   OpenGL/  │     PTY      │  │
│  │  Render  │   Vulkan   │   Backend    │  │
│  └──────────┴────────────┴──────────────┘  │
│                     │                        │
│            ┌────────▼──────────┐            │
│            │ KDE Frameworks    │            │
│            │  Integration      │            │
│            └───────────────────┘            │
└─────────────────────────────────────────────┘
```

## Components

### 1. Kirigami UI Layer (`kurrent-kirigami/`)

**Technology**: Qt 6 + QML + Kirigami framework  
**Language**: C++, QML  
**Purpose**: Native KDE Plasma UI chrome

**Features**:
- Window decorations (native KDE style)
- Settings dialog (Kirigami-based)
- Menu bar and context menus
- Tab bar (if using Kirigami tabs)
- System tray integration

**Files**:
- `qml/main.qml` - Main application window
- `qml/SettingsPage.qml` - Settings UI
- `src/terminalbridge.{h,cpp}` - Qt/Rust bridge
- `CMakeLists.txt` - Qt/KDE build configuration

### 2. Rust Terminal Core (`kurrent-gui/`)

**Technology**: Rust + OpenGL/Vulkan  
**Purpose**: High-performance terminal rendering

**Features**:
- Terminal emulation (VT sequences)
- GPU-accelerated text rendering
- PTY management
- Input handling (when not using Kirigami)

**Integration Points**:
- Exposes FFI functions for Qt bridge
- Can be embedded in Qt Quick item
- Shares window/OpenGL context with Qt

### 3. KDE Frameworks Integration (`crates/kde-frameworks/`)

**Technology**: Rust + DBus (zbus)  
**Purpose**: Deep KDE Plasma integration

**Modules**:
- `color_scheme` - KDE color detection and theming
- `notifications` - KNotifications (native notifications)
- `wallet` - KWallet (secure credential storage)
- `activities` - Plasma Activities support
- `dbus` - DBus service communication

## Data Flow

### Configuration Changes

```
QML Settings → TerminalBridge (C++) → FFI → Rust Config → Terminal Render
```

### Terminal Output

```
PTY → Rust Terminal Parser → OpenGL Rendering → Qt Surface
```

### KDE Color Scheme Change

```
DBus Signal → kde-frameworks → TerminalBridge → QML + Rust
```

## Build Process

### Option 1: Separate Binaries

Build `kurrent-gui` (Rust) and `kurrent-kirigami-ui` (Qt) as separate executables:

```bash
# Build Rust terminal
cargo build --release -p kurrent-gui

# Build Kirigami UI
cd kurrent-kirigami
cmake -B build
cmake --build build

# Run
./kurrent-kirigami/build/kurrent-kirigami-ui
```

The Qt application spawns/embeds the Rust process.

### Option 2: Shared Library

Build Rust terminal as a shared library (`.so`) that Qt loads:

```bash
# Build Rust as library
cargo build --release --lib -p kurrent-gui

# Build Qt with library linkage
cd kurrent-kirigami
cmake -B build -DKURRENT_LIB=/path/to/libkurrent_gui.so
cmake --build build
```

### Option 3: Hybrid (Recommended)

Use Rust for headless terminal + OpenGL, Qt for window system:

1. Rust provides OpenGL rendering surface
2. Qt provides window decorations and chrome
3. Share OpenGL context between Rust and Qt
4. Use FFI for configuration and control

## Implementation Phases

### Phase 1: Keep Current Architecture
- Continue with pure Rust windowing (X11/Wayland)
- Add `kde-frameworks` crate for KDE integration
- Get basic GUI working first

### Phase 2: Add Kirigami Settings
- Build Kirigami settings dialog as separate app
- Launch from Rust terminal
- Share config via files or IPC

### Phase 3: Hybrid Window
- Embed Rust OpenGL in Qt Quick item
- Use Kirigami for window chrome only
- Terminal rendering stays in Rust

### Phase 4: Full Integration
- Complete FFI bridge
- Seamless integration
- All KDE features working

## Advantages of Hybrid Approach

**Rust Strengths**:
- High-performance rendering
- Low-level PTY control
- Minimal dependencies
- Cross-platform core

**Qt/Kirigami Strengths**:
- Native KDE look and feel
- Rich UI components
- System integration
- Accessibility

## FFI Interface Design

### Rust Side (C ABI)

```rust
#[no_mangle]
pub extern "C" fn kurrent_terminal_new() -> *mut TerminalState {
    // Create new terminal instance
}

#[no_mangle]
pub extern "C" fn kurrent_terminal_send_input(
    handle: *mut TerminalState,
    data: *const u8,
    len: usize
) {
    // Send input to PTY
}

#[no_mangle]
pub extern "C" fn kurrent_terminal_get_opengl_texture() -> u32 {
    // Return OpenGL texture ID for rendering
}
```

### Qt Side (C++)

```cpp
extern "C" {
    void* kurrent_terminal_new();
    void kurrent_terminal_destroy(void* handle);
    void kurrent_terminal_send_input(void* handle, const char* data, size_t len);
    unsigned int kurrent_terminal_get_opengl_texture();
}
```

## Configuration

Both layers share configuration via:
- Lua files (`~/.config/kurrent/kurrent.lua`)
- KDE config system (via `kde-frameworks` crate)
- IPC/shared memory for runtime changes

## Next Steps

See `NEXT_SESSION.md` for implementation guide.
