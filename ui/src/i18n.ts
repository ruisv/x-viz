import { createSignal } from "solid-js";

export type Locale = "zh-CN" | "en-US";

const translations = {
  "zh-CN": {
    // App
    "app.fetchLatest": "获取最新壁纸",
    "app.fetching": "获取中…",
    "app.fetchTooltip": "从 Bing 获取最新壁纸并设置",
    "app.gallery": "画廊",
    "app.settings": "设置",
    "app.running": "运行中",

    // Gallery
    "gallery.title": "壁纸画廊",
    "gallery.count": "张",
    "gallery.empty": "暂无壁纸，点击右上角「获取最新壁纸」开始",
    "gallery.setWallpaper": "设为壁纸",
    "gallery.delete": "删除",
    "gallery.wallpaperSet": "壁纸设置成功",
    "gallery.wallpaperSetFail": "壁纸设置失败",
    "gallery.deleted": "壁纸已删除",
    "gallery.deleteFail": "删除失败",

    // Settings
    "settings.source": "壁纸源",
    "settings.region": "地区",
    "settings.regionDesc": "不同地区可能有不同的每日壁纸",
    "settings.fetchCount": "获取数量",
    "settings.fetchCountDesc": "每次更新获取的壁纸张数（1–8）",
    "settings.resolution": "分辨率",
    "settings.resolutionDesc": "UHD 可获取 4K 超高清壁纸",
    "settings.storage": "存储",
    "settings.storagePath": "存储路径",
    "settings.storagePathDesc": "壁纸文件的下载位置",
    "settings.choose": "选择",
    "settings.retentionDays": "保留天数",
    "settings.retentionDaysDesc": "超出天数的壁纸将自动清理",
    "settings.general": "通用",
    "settings.language": "界面语言",
    "settings.languageDesc": "应用界面的显示语言",
    "settings.updateInterval": "更新间隔",
    "settings.updateIntervalDesc": "壁纸自动检查更新的频率",
    "settings.autoStart": "开机自启动",
    "settings.autoStartDesc": "登录时自动启动 x-viz",
    "settings.silentStart": "静默启动",
    "settings.silentStartDesc": "启动时隐藏主窗口，仅显示托盘图标",
    "settings.saved": "设置已自动保存",
    "settings.about": "基于 Tauri + Solid.js 构建",

    // Theme
    "settings.theme": "外观主题",
    "settings.themeDesc": "跟随系统或手动切换明暗模式",
    "theme.auto": "跟随系统",
    "theme.light": "浅色",
    "theme.dark": "深色",

    // Intervals
    "interval.daily": "每天",
    "interval.hourly": "每小时",
    "interval.manual": "手动",
  },
  "en-US": {
    "app.fetchLatest": "Fetch Latest",
    "app.fetching": "Fetching…",
    "app.fetchTooltip": "Fetch latest wallpaper from Bing and apply",
    "app.gallery": "Gallery",
    "app.settings": "Settings",
    "app.running": "Running",

    "gallery.title": "Wallpaper Gallery",
    "gallery.count": "items",
    "gallery.empty": 'No wallpapers yet. Click "Fetch Latest" in the top-right corner to get started.',
    "gallery.setWallpaper": "Set as Wallpaper",
    "gallery.delete": "Delete",
    "gallery.wallpaperSet": "Wallpaper set successfully",
    "gallery.wallpaperSetFail": "Failed to set wallpaper",
    "gallery.deleted": "Wallpaper deleted",
    "gallery.deleteFail": "Failed to delete wallpaper",

    "settings.source": "Wallpaper Source",
    "settings.region": "Region",
    "settings.regionDesc": "Different regions may have different daily wallpapers",
    "settings.fetchCount": "Fetch Count",
    "settings.fetchCountDesc": "Number of wallpapers to fetch per update (1–8)",
    "settings.resolution": "Resolution",
    "settings.resolutionDesc": "UHD provides 4K ultra-high-definition wallpapers",
    "settings.storage": "Storage",
    "settings.storagePath": "Storage Path",
    "settings.storagePathDesc": "Download location for wallpaper files",
    "settings.choose": "Browse",
    "settings.retentionDays": "Retention Days",
    "settings.retentionDaysDesc": "Wallpapers older than this will be cleaned up",
    "settings.general": "General",
    "settings.language": "Language",
    "settings.languageDesc": "Display language for the app interface",
    "settings.updateInterval": "Update Interval",
    "settings.updateIntervalDesc": "How often to check for new wallpapers",
    "settings.autoStart": "Launch at Login",
    "settings.autoStartDesc": "Automatically start x-viz on login",
    "settings.silentStart": "Silent Start",
    "settings.silentStartDesc": "Hide main window on startup, show tray icon only",
    "settings.saved": "Settings saved",
    "settings.about": "Built with Tauri + Solid.js",

    "settings.theme": "Appearance",
    "settings.themeDesc": "Follow system or manually switch light/dark mode",
    "theme.auto": "Auto",
    "theme.light": "Light",
    "theme.dark": "Dark",

    "interval.daily": "Daily",
    "interval.hourly": "Hourly",
    "interval.manual": "Manual",
  },
} as const;

type TranslationKey = keyof (typeof translations)["zh-CN"];

const [locale, setLocale] = createSignal<Locale>("zh-CN");

export function initLocale(lang: string) {
  if (lang.startsWith("en")) {
    setLocale("en-US");
  } else {
    setLocale("zh-CN");
  }
}

export function t(key: TranslationKey): string {
  return translations[locale()][key] ?? key;
}

export { locale, setLocale };
