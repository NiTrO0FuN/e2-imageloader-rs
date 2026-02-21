import { quality, size } from "../store/options";
import { invoke } from "@tauri-apps/api/core";
import { createToast, Toast, ToastSeverity } from "./toasts";

async function process(path: string) {
  try {
    await invoke("process_image", { path, quality: quality.value, chunkSize: size.value });
    createToast(new Toast("image.select-success", ToastSeverity.SUCCESS));
  } catch (error) {
    createToast(new Toast("image.select-error." + error, ToastSeverity.ALERT));
  }
}

export { process };
