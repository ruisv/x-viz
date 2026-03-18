import { Component, createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { IconGlobe, IconFolder, IconGear } from "./Icons";
import { t, setLocale, type Locale } from "../i18n";
import { setTheme, type ThemeMode } from "../theme";
import "../styles/settings.css";

export interface AppConfig {
  region: string;
  storage_path: string;
  retention_days: number;
  auto_start: boolean;
  update_interval: string;
  resolution: string;
  fetch_count: number;
  silent_start: boolean;
  language: string;
  theme: string;
}

const DEFAULT_CONFIG: AppConfig = {
  region: "en-US",
  storage_path: "",
  retention_days: 30,
  auto_start: false,
  update_interval: "daily",
  resolution: "UHD",
  fetch_count: 1,
  silent_start: false,
  language: "zh-CN",
  theme: "auto",
};

const REGIONS = [
  { value: "en-US", label: "English (US)" },
  { value: "zh-CN", label: "中文 (中国)" },
  { value: "ja-JP", label: "日本語" },
  { value: "de-DE", label: "Deutsch" },
  { value: "fr-FR", label: "Français" },
  { value: "en-GB", label: "English (UK)" },
  { value: "en-AU", label: "English (AU)" },
  { value: "en-IN", label: "English (IN)" },
];

const LANGUAGES = [
  { value: "zh-CN", label: "中文" },
  { value: "en-US", label: "English" },
];

const RESOLUTIONS = [
  { value: "UHD", label: "4K UHD" },
  { value: "1920x1080", label: "1080p" },
];

let saveTimer: ReturnType<typeof setTimeout> | null = null;

const Settings: Component = () => {
  const [config, setConfig] = createSignal<AppConfig>(DEFAULT_CONFIG);
  const [saved, setSaved] = createSignal(false);

  onMount(async () => {
    try {
      const cfg = await invoke<AppConfig>("get_config");
      setConfig(cfg);
    } catch (err) {
      console.error("Failed to load config:", err);
    }
  });

  const autoSave = (newConfig: AppConfig) => {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(async () => {
      try {
        await invoke("save_config", { config: newConfig });
        setSaved(true);
        setTimeout(() => setSaved(false), 1500);
      } catch (err) {
        console.error("Failed to save config:", err);
      }
    }, 300);
  };

  const updateField = <K extends keyof AppConfig>(key: K, value: AppConfig[K]) => {
    const newConfig = { ...config(), [key]: value };
    setConfig(newConfig);
    autoSave(newConfig);
  };

  const updateLanguage = (lang: string) => {
    setLocale(lang as Locale);
    updateField("language", lang);
  };

  const updateTheme = (theme: string) => {
    setTheme(theme as ThemeMode);
    updateField("theme", theme);
  };

  const choosePath = async () => {
    try {
      const path = await invoke<string | null>("choose_storage_path");
      if (path) updateField("storage_path", path);
    } catch (err) {
      console.error("Failed to open folder picker:", err);
    }
  };

  return (
    <div class="settings">
      {/* --- Source --- */}
      <div class="settings-section">
        <div class="settings-section-header">
          <span class="settings-section-icon"><IconGlobe /></span>
          {t("settings.source")}
        </div>
        <div class="settings-row">
          <div class="settings-row-info">
            <div class="settings-row-label">{t("settings.region")}</div>
            <div class="settings-row-desc">{t("settings.regionDesc")}</div>
          </div>
          <div class="settings-row-control">
            <select
              class="select"
              value={config().region}
              onChange={(e) => updateField("region", e.currentTarget.value)}
            >
              {REGIONS.map((r) => (
                <option value={r.value}>{r.label}</option>
              ))}
            </select>
          </div>
        </div>
        <div class="settings-row">
          <div class="settings-row-info">
            <div class="settings-row-label">{t("settings.fetchCount")}</div>
            <div class="settings-row-desc">{t("settings.fetchCountDesc")}</div>
          </div>
          <div class="settings-row-control">
            <input
              class="input input-sm"
              type="number"
              min="1"
              max="8"
              value={config().fetch_count}
              onInput={(e) =>
                updateField(
                  "fetch_count",
                  Math.max(1, Math.min(8, parseInt(e.currentTarget.value) || 1))
                )
              }
            />
          </div>
        </div>
        <div class="settings-row">
          <div class="settings-row-info">
            <div class="settings-row-label">{t("settings.resolution")}</div>
            <div class="settings-row-desc">{t("settings.resolutionDesc")}</div>
          </div>
          <div class="settings-row-control">
            <select
              class="select"
              value={config().resolution}
              onChange={(e) => updateField("resolution", e.currentTarget.value)}
            >
              {RESOLUTIONS.map((r) => (
                <option value={r.value}>{r.label}</option>
              ))}
            </select>
          </div>
        </div>
      </div>

      {/* --- Storage --- */}
      <div class="settings-section">
        <div class="settings-section-header">
          <span class="settings-section-icon"><IconFolder /></span>
          {t("settings.storage")}
        </div>
        <div class="settings-row">
          <div class="settings-row-info">
            <div class="settings-row-label">{t("settings.storagePath")}</div>
            <div class="settings-row-desc">{t("settings.storagePathDesc")}</div>
          </div>
          <div class="settings-row-control">
            <div class="path-input-group">
              <input
                class="input"
                type="text"
                value={config().storage_path}
                placeholder="—"
                readonly
              />
              <button class="btn btn-sm" onClick={choosePath}>
                {t("settings.choose")}
              </button>
            </div>
          </div>
        </div>
        <div class="settings-row">
          <div class="settings-row-info">
            <div class="settings-row-label">{t("settings.retentionDays")}</div>
            <div class="settings-row-desc">{t("settings.retentionDaysDesc")}</div>
          </div>
          <div class="settings-row-control">
            <input
              class="input input-sm"
              type="number"
              min="1"
              max="365"
              value={config().retention_days}
              onInput={(e) =>
                updateField("retention_days", parseInt(e.currentTarget.value) || 30)
              }
            />
          </div>
        </div>
      </div>

      {/* --- General --- */}
      <div class="settings-section">
        <div class="settings-section-header">
          <span class="settings-section-icon"><IconGear /></span>
          {t("settings.general")}
        </div>
        <div class="settings-row">
          <div class="settings-row-info">
            <div class="settings-row-label">{t("settings.language")}</div>
            <div class="settings-row-desc">{t("settings.languageDesc")}</div>
          </div>
          <div class="settings-row-control">
            <select
              class="select"
              value={config().language}
              onChange={(e) => updateLanguage(e.currentTarget.value)}
            >
              {LANGUAGES.map((l) => (
                <option value={l.value}>{l.label}</option>
              ))}
            </select>
          </div>
        </div>
        <div class="settings-row">
          <div class="settings-row-info">
            <div class="settings-row-label">{t("settings.theme")}</div>
            <div class="settings-row-desc">{t("settings.themeDesc")}</div>
          </div>
          <div class="settings-row-control">
            <select
              class="select"
              value={config().theme}
              onChange={(e) => updateTheme(e.currentTarget.value)}
            >
              <option value="auto">{t("theme.auto")}</option>
              <option value="light">{t("theme.light")}</option>
              <option value="dark">{t("theme.dark")}</option>
            </select>
          </div>
        </div>
        <div class="settings-row">
          <div class="settings-row-info">
            <div class="settings-row-label">{t("settings.updateInterval")}</div>
            <div class="settings-row-desc">{t("settings.updateIntervalDesc")}</div>
          </div>
          <div class="settings-row-control">
            <select
              class="select"
              value={config().update_interval}
              onChange={(e) => updateField("update_interval", e.currentTarget.value)}
            >
              <option value="daily">{t("interval.daily")}</option>
              <option value="hourly">{t("interval.hourly")}</option>
              <option value="manual">{t("interval.manual")}</option>
            </select>
          </div>
        </div>
        <div class="settings-row">
          <div class="settings-row-info">
            <div class="settings-row-label">{t("settings.autoStart")}</div>
            <div class="settings-row-desc">{t("settings.autoStartDesc")}</div>
          </div>
          <div class="settings-row-control">
            <label class="toggle">
              <input
                type="checkbox"
                checked={config().auto_start}
                onChange={(e) => updateField("auto_start", e.currentTarget.checked)}
              />
              <span class="toggle-slider" />
            </label>
          </div>
        </div>
        <div class="settings-row">
          <div class="settings-row-info">
            <div class="settings-row-label">{t("settings.silentStart")}</div>
            <div class="settings-row-desc">{t("settings.silentStartDesc")}</div>
          </div>
          <div class="settings-row-control">
            <label class="toggle">
              <input
                type="checkbox"
                checked={config().silent_start}
                onChange={(e) => updateField("silent_start", e.currentTarget.checked)}
              />
              <span class="toggle-slider" />
            </label>
          </div>
        </div>
      </div>

      <div class="settings-save-indicator" classList={{ visible: saved() }}>
        {t("settings.saved")}
      </div>

      <div class="settings-about">
        <span class="settings-about-version">x-viz v0.1.0</span>
        {" · "}{t("settings.about")}
      </div>
    </div>
  );
};

export default Settings;
