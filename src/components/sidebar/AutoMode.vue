<template>
  <button :class="{ on: automode }" @click="toggleAutomode">{{ $t("sidebar.automode") }}</button>
</template>

<script lang="ts" setup>
import { Event, UnlistenFn, listen } from "@tauri-apps/api/event";
import { Store } from "@tauri-apps/plugin-store";
import { onMounted, onUnmounted, ref, watch } from "vue";
import { createToast, ToastSeverity } from "../../utils/toasts";

const automode = ref(false);
let unlisten: UnlistenFn | undefined;

let store: Store;
onMounted(async () => {
  store = await Store.load("settings.json");
  automode.value = (await store.get<boolean>("automode")) || false;
});

onUnmounted(() => {
  unlisten;
});

async function toggleAutomode() {
  automode.value = !automode.value;
  store.set("automode", automode.value);
  createToast("automode." + (automode.value ? "on" : "off"), ToastSeverity.INFO);
}

watch(automode, async (on) => {
  console.log(on);
  if (unlisten) {
    unlisten();
    unlisten = undefined;
  }
  if (on) {
    unlisten = await listen<NotificationOutcome>("automode", handleNotification);
  }
});

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
