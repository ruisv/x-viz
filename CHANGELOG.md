# Changelog

All notable changes to x-viz will be documented in this file.

## [v0.1.0] - 2026-03-19

### 核心功能

- 自动获取 Bing 每日壁纸并设为桌面壁纸
- 支持 20+ 地区壁纸源切换（zh-CN、en-US、ja-JP 等）
- 4K/UHD 超高清壁纸下载
- 后台定时自动更新（每小时 / 每天可配置）
- 可配置单次获取数量（1-8 张），按日期自动去重
- LRU 自动清理，可配置壁纸保留天数

### 跨平台支持

- macOS Apple Silicon (aarch64)
- macOS Intel (x86_64)
- Windows x86_64
- Linux x86_64（GNOME、KDE、XFCE、MATE、Sway、Hyprland）

### 系统集成

- 系统托盘常驻后台，无需保持窗口打开
- 开机自启动（可在设置中开关）
- 关闭窗口自动销毁 WebView，节省约 50-80MB 内存
- 首次启动自动显示主窗口，后续静默启动

### 界面

- 壁纸画廊：网格浏览、一键设为壁纸、删除管理
- 设置面板：地区、存储路径、保留天数、自启动、更新间隔、分辨率
- 中英双语界面，自动检测系统语言，支持手动切换
- macOS 原生 Liquid Glass 风格 UI

### 下载说明

| 平台 | 文件 |
|------|------|
| macOS (Apple Silicon) | `x-viz_*_aarch64.dmg` |
| macOS (Intel) | `x-viz_*_x64.dmg` |
| Windows | `x-viz_*_x64-setup.exe` 或 `x-viz_*_x64_en-US.msi` |
| Linux (Debian/Ubuntu) | `x-viz_*_amd64.deb` |
| Linux (通用) | `x-viz_*_amd64.AppImage` |
