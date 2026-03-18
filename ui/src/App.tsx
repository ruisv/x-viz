import { createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import Gallery from "./components/Gallery";
import Settings, { type AppConfig } from "./components/Settings";
import Toast from "./components/Toast";
import { AppLogo, IconGrid, IconSettings, IconDownload } from "./components/Icons";
import { t, initLocale } from "./i18n";
import { initTheme, type ThemeMode } from "./theme";
import "./App.css";

type Tab = "gallery" | "settings";

function App() {
  const [activeTab, setActiveTab] = createSignal<Tab>("gallery");
  const [updating, setUpdating] = createSignal(false);
  const [isWindows, setIsWindows] = createSignal(false);

  onMount(async () => {
    try {
      const cfg = await invoke<AppConfig>("get_config");
      initLocale(cfg.language);
      initTheme((cfg as any).theme as ThemeMode || "auto");
    } catch (_) {}
    try {
      const info = await invoke<{ os: string }>("get_system_info");
      setIsWindows(info.os === "windows");
    } catch (_) {}
  });

  const handleUpdate = async () => {
    setUpdating(true);
    try {
      await invoke("fetch_wallpapers");
    } catch (err) {
      console.error("Failed to fetch wallpapers:", err);
    } finally {
      setUpdating(false);
    }
  };

  return (
    <div class="app">
      {!isWindows() && (
        <header class="app-header" data-tauri-drag-region>
          <div class="app-title">
            <span class="app-title-icon"><AppLogo /></span>
            x-viz
          </div>
          <div class="app-header-actions">
            <button
              class="btn btn-ghost btn-sm"
              onClick={handleUpdate}
              disabled={updating()}
              title={t("app.fetchTooltip")}
            >
              <IconDownload /> {updating() ? t("app.fetching") : t("app.fetchLatest")}
            </button>
          </div>
        </header>
      )}
      {isWindows() && (
        <div class="app-header-windows">
          <button
            class="btn btn-ghost btn-sm"
            onClick={handleUpdate}
            disabled={updating()}
            title={t("app.fetchTooltip")}
          >
            <IconDownload /> {updating() ? t("app.fetching") : t("app.fetchLatest")}
          </button>
        </div>
      )}

      <nav class="app-nav">
        <div class="tab-bar">
          <button
            class={`tab ${activeTab() === "gallery" ? "active" : ""}`}
            onClick={() => setActiveTab("gallery")}
          >
            <IconGrid /> {t("app.gallery")}
          </button>
          <button
            class={`tab ${activeTab() === "settings" ? "active" : ""}`}
            onClick={() => setActiveTab("settings")}
          >
            <IconSettings /> {t("app.settings")}
          </button>
        </div>
      </nav>

      <main class="app-content">
        {activeTab() === "gallery" ? <Gallery /> : <Settings />}
      </main>

      <footer class="app-status">
        <div>
          <span class="status-dot" />
          {t("app.running")}
        </div>
        <div>v0.1.0</div>
      </footer>

      <Toast />
    </div>
  );
}

export default App;
