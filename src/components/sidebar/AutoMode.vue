<template>
  <button :class="{ on: options.automode }" @click="toggleAutomode">{{ $t("sidebar.automode") }}</button>
</template>

<script lang="ts" setup>
import { Event, UnlistenFn, listen } from "@tauri-apps/api/event";
import { options } from "../../store/options";
import { onUnmounted, watch } from "vue";
import { createToast, ToastSeverity } from "../../utils/toasts";

let unlisten: UnlistenFn | undefined;

onUnmounted(() => {
  unlisten;
});

async function toggleAutomode() {
  options.value.automode = !options.value.automode;
  createToast("automode." + (options.value.automode ? "on" : "off"), ToastSeverity.INFO);
}

watch(
  () => options.value.automode,
  async (on) => {
    if (unlisten) {
      unlisten();
      unlisten = undefined;
    }
    if (on) {
      unlisten = await listen<NotificationOutcome>("automode", handleNotification);
    }
  },
);

enum NotificationOutcome {
  SUCCESS = "success",
  ERROR = "error",
}

function handleNotification(event: Event<NotificationOutcome>) {
  const outcome = event.payload;
  createToast(
    "automode." + outcome,
    outcome == NotificationOutcome.SUCCESS ? ToastSeverity.SUCCESS : ToastSeverity.ALERT,
  );
}
</script>

<style scoped>
button.on {
  background-color: var(--positive-color);
}
</style>
