import { reactive, Reactive } from "vue";

const MAX_TOASTS = 3;
const TOAST_DURATION = 5;

class Toast {
  text: string;
  severity: ToastSeverity;
  timestamp: number;

  constructor(text: string, severity: ToastSeverity) {
    this.text = text;
    this.severity = severity;
    this.timestamp = Date.now();
  }
}

const toasts: Reactive<Toast[]> = reactive([]);

function createToast(toast: Toast) {
  if (toasts.length >= MAX_TOASTS) toasts.shift();
  toasts.push(toast);

  setTimeout(() => {
    const index = toasts.indexOf(toast);
    if (index > -1) removeToast(index);
  }, TOAST_DURATION * 1000);
}

function removeToast(i: number) {
  toasts.splice(i, 1);
}

enum ToastSeverity {
  INFO = "info",
  SUCCESS = "success",
  ALERT = "alert",
}

export { createToast, removeToast, toasts, ToastSeverity, Toast };
