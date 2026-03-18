// Global toast notification system.
// Listens for `update-result` events from the Rust backend and renders
// a list of transient notifications in the bottom-right corner.

import { Component, createSignal, For, onCleanup, onMount } from "solid-js";
import { listen } from "@tauri-apps/api/event";
import { IconCheckCircle, IconXCircle } from "./Icons";
import "../styles/toast.css";

interface ToastItem {
  id: number;
  type: "success" | "error";
  message: string;
}

interface UpdateResult {
  success: boolean;
  message: string;
}

let nextId = 0;
const [toasts, setToasts] = createSignal<ToastItem[]>([]);

function addToast(type: "success" | "error", message: string) {
  const id = ++nextId;
  setToasts((prev) => [...prev, { id, type, message }]);
  setTimeout(() => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  }, 4000);
}

const Toast: Component = () => {
  onMount(async () => {
    const unlisten = await listen<UpdateResult>("update-result", (event) => {
      const { success, message } = event.payload;
      addToast(success ? "success" : "error", message);
    });
    onCleanup(unlisten);
  });

  return (
    <div class="toast-container">
      <For each={toasts()}>
        {(toast) => (
          <div class={`toast toast-${toast.type}`}>
            <span class="toast-icon">
              {toast.type === "success" ? <IconCheckCircle /> : <IconXCircle />}
            </span>
            <span class="toast-message">{toast.message}</span>
          </div>
        )}
      </For>
    </div>
  );
};

export { addToast };
export default Toast;
