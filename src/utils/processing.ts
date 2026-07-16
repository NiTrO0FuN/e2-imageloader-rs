import { invoke } from "@tauri-apps/api/core";
import { createToast, ToastSeverity } from "./toasts";

async function process(path: string) {
  try {
    await invoke("process_image", { path });
    createToast("image.select-success", ToastSeverity.SUCCESS);
  } catch (error) {
    createToast("image.select-error." + error, ToastSeverity.ALERT);
  }
}

export { process };
