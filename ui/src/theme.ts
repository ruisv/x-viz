import { createSignal } from "solid-js";

export type ThemeMode = "auto" | "light" | "dark";
type ResolvedTheme = "light" | "dark";

const [themeMode, setThemeMode] = createSignal<ThemeMode>("auto");

function getSystemTheme(): ResolvedTheme {
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";
}

function applyTheme(mode: ThemeMode) {
  const resolved: ResolvedTheme = mode === "auto" ? getSystemTheme() : mode;
  document.documentElement.setAttribute("data-theme", resolved);
}

/** Initialize theme and listen for system changes. */
export function initTheme(mode: ThemeMode) {
  setThemeMode(mode);
  applyTheme(mode);

  // Listen for system theme changes (only matters in auto mode)
  window
    .matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", () => {
      if (themeMode() === "auto") {
        applyTheme("auto");
      }
    });
}

/** Change theme mode and apply immediately. */
export function setTheme(mode: ThemeMode) {
  setThemeMode(mode);
  applyTheme(mode);
}

export { themeMode };
