# x-viz

A lightweight, cross-platform desktop wallpaper manager that automatically fetches and applies Bing's daily images.

[中文文档](./docs/README_zh.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## Features

- Automatically fetch and set Bing daily wallpapers
- Multi-region wallpaper sources (en-US, zh-CN, ja-JP, etc.)
- 4K UHD resolution support
- Batch fetch up to 8 recent wallpapers
- Gallery management with preview, apply and delete
- Custom storage path and auto-cleanup
- System tray background operation
- Launch at login with silent start option
- Chinese / English UI

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Framework | [Tauri v2](https://tauri.app/) |
| Backend | Rust |
| Frontend | [Solid.js](https://solidjs.com/) + TypeScript |
| Build | [Vite](https://vite.dev/) |
| Styling | Vanilla CSS (macOS Liquid Glass) |

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) >= 1.75
- Platform dependencies per [Tauri docs](https://tauri.app/start/prerequisites/)

### Development

```bash
# Install dependencies
npm install

# Start dev mode
npm run tauri dev

# Production build
npm run tauri build
```

## Project Structure

```
x-viz/
├── ui/                     # Frontend (Solid.js + TypeScript)
│   └── src/
│       ├── components/     # UI components
│       ├── styles/         # Modular stylesheets
│       ├── i18n.ts         # Internationalization
│       ├── App.tsx         # App shell
│       └── index.css       # Design system
├── src-tauri/              # Backend (Rust)
│   └── src/
│       ├── platform/       # Cross-platform adapters
│       ├── bing.rs         # Bing API client
│       ├── config.rs       # Configuration persistence
│       ├── storage.rs      # Wallpaper file management
│       ├── scheduler.rs    # Background update scheduler
│       └── commands.rs     # Tauri IPC commands
└── scripts/                # Build utilities
```

## Platform Support

| Platform | Wallpaper API |
|----------|--------------|
| macOS | AppleScript (`osascript`) |
| Windows | `SystemParametersInfoW` |
| Linux | GNOME / KDE / XFCE CLI tools |

## License

[MIT](LICENSE)
