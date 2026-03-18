# x-viz

一款轻量级跨平台桌面壁纸管理工具，自动获取并设置 Bing 每日精选图片。

[English](../README.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../LICENSE)

---

## 功能

- 自动获取并设置 Bing 每日壁纸
- 多地区壁纸源切换（en-US、zh-CN、ja-JP 等）
- 支持 4K 超高清分辨率
- 批量获取最近 8 天壁纸
- 壁纸画廊管理：预览、应用、删除
- 自定义存储路径与自动清理
- 系统托盘后台运行
- 开机自启动与静默启动
- 中英双语界面

## 技术栈

| 层级 | 技术 |
|------|-----|
| 框架 | [Tauri v2](https://tauri.app/) |
| 后端 | Rust |
| 前端 | [Solid.js](https://solidjs.com/) + TypeScript |
| 构建 | [Vite](https://vite.dev/) |
| 样式 | Vanilla CSS (macOS Liquid Glass) |

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) >= 1.75
- 系统依赖见 [Tauri 文档](https://tauri.app/start/prerequisites/)

### 开发

```bash
# 安装依赖
npm install

# 启动开发模式
npm run tauri dev

# 生产构建
npm run tauri build
```

## 项目结构

```
x-viz/
├── ui/                     # 前端 (Solid.js + TypeScript)
│   └── src/
│       ├── components/     # UI 组件
│       ├── styles/         # 模块化样式
│       ├── i18n.ts         # 国际化
│       ├── App.tsx         # 应用主框架
│       └── index.css       # 设计系统
├── src-tauri/              # 后端 (Rust)
│   └── src/
│       ├── platform/       # 跨平台适配层
│       ├── bing.rs         # Bing API 客户端
│       ├── config.rs       # 配置持久化
│       ├── storage.rs      # 壁纸文件管理
│       ├── scheduler.rs    # 后台更新调度
│       └── commands.rs     # Tauri IPC 命令
└── scripts/                # 构建工具
```

## 支持平台

| 平台 | 壁纸设置方式 |
|------|------------|
| macOS | AppleScript (`osascript`) |
| Windows | `SystemParametersInfoW` |
| Linux | GNOME / KDE / XFCE 命令行工具 |

## 许可证

[MIT](../LICENSE)
