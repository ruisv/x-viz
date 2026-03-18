import { Component, createSignal, For, onCleanup, onMount, Show } from "solid-js";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { IconSetWallpaper, IconTrash, IconImageOff } from "./Icons";
import { addToast } from "./Toast";
import { t } from "../i18n";
import "../styles/gallery.css";

export interface WallpaperItem {
  title: string;
  date: string;
  path: string;
  copyright: string;
}

const Gallery: Component = () => {
  const [wallpapers, setWallpapers] = createSignal<WallpaperItem[]>([]);
  const [loading, setLoading] = createSignal(false);

  const fetchWallpapers = async () => {
    setLoading(true);
    try {
      const result = await invoke<WallpaperItem[]>("get_gallery");
      setWallpapers(result);
    } catch (err) {
      console.error("Failed to fetch gallery:", err);
    } finally {
      setLoading(false);
    }
  };

  const setAsWallpaper = async (path: string) => {
    try {
      await invoke("set_wallpaper_now", { path });
      addToast("success", t("gallery.wallpaperSet"));
    } catch (err) {
      console.error("Failed to set wallpaper:", err);
      addToast("error", t("gallery.wallpaperSetFail"));
    }
  };

  const deleteWallpaper = async (path: string) => {
    try {
      await invoke("delete_wallpaper", { path });
      setWallpapers((prev) => prev.filter((wp) => wp.path !== path));
      addToast("success", t("gallery.deleted"));
    } catch (err) {
      console.error("Failed to delete wallpaper:", err);
      addToast("error", t("gallery.deleteFail"));
    }
  };

  onMount(async () => {
    fetchWallpapers();
    const unlistenUpdate = await listen("wallpapers-updated", () => fetchWallpapers());
    const unlistenFocus = await getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (focused) fetchWallpapers();
    });
    onCleanup(() => {
      unlistenUpdate();
      unlistenFocus();
    });
  });

  return (
    <div class="gallery">
      <div class="gallery-header">
        <div class="gallery-title">
          <h2>{t("gallery.title")}</h2>
          <span class="gallery-count">{wallpapers().length} {t("gallery.count")}</span>
        </div>
      </div>

      <Show
        when={wallpapers().length > 0}
        fallback={
          <div class="gallery-empty">
            <div class="gallery-empty-icon"><IconImageOff /></div>
            <p class="gallery-empty-text">{t("gallery.empty")}</p>
          </div>
        }
      >
        <div class="gallery-grid">
          <For each={wallpapers()}>
            {(wp) => (
              <div class="wallpaper-card">
                <img
                  class="wallpaper-thumb"
                  src={convertFileSrc(wp.path)}
                  alt={wp.title}
                  loading="lazy"
                />
                <div class="wallpaper-info">
                  <div class="wallpaper-title" title={wp.title}>{wp.title}</div>
                  <div class="wallpaper-date">{wp.date}</div>
                </div>
                <div class="wallpaper-overlay">
                  <button
                    class="btn btn-primary btn-sm"
                    onClick={() => setAsWallpaper(wp.path)}
                  >
                    <IconSetWallpaper /> {t("gallery.setWallpaper")}
                  </button>
                  <button
                    class="btn btn-danger btn-sm"
                    onClick={() => deleteWallpaper(wp.path)}
                  >
                    <IconTrash /> {t("gallery.delete")}
                  </button>
                </div>
              </div>
            )}
          </For>
        </div>
      </Show>
    </div>
  );
};

export default Gallery;
