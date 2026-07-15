<template>
  <button id="gmod_folder" :class="{ unset: !path }" @click="chooseFolder">
    <img src="/gmod_logo.svg" width="32" height="32" style="display: block" />
  </button>
</template>

<script lang="ts" setup>
import { Store } from "@tauri-apps/plugin-store";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { createToast, ToastSeverity } from "../../utils/toasts";
import { ref, onMounted } from "vue";

const path = ref("");
let store: Store;

onMounted(async () => {
  store = await Store.load("settings.json");
  path.value = (await store.get<string>("gamePath")) || "";
  store.onKeyChange<string>("gamePath", (gamePath) => (path.value = gamePath || ""));
});

async function chooseFolder() {
  const directory = await open({
    directory: true,
    defaultPath: path.value,
  });
  if (!directory) return;
  let valid = await invoke("is_valid_game_path", { path: directory });
  if (valid) {
    await store.set("gamePath", directory);
    createToast("navbar.gamePath.success", ToastSeverity.SUCCESS);
  }
}
</script>

<style>
#gmod_folder {
  height: fit-content;
  margin-right: 9px;
  border-radius: 5px;

  &.unset {
    animation: pulse 2s ease-in-out infinite;
  }
}

@keyframes pulse {
  0%,
  100% {
    transform: scale(1);
    box-shadow: 0 0 10px 0 rgba(255, 255, 255, 0.7);
  }
  50% {
    transform: scale(1.05);
    box-shadow: 0 0 15px 5px rgba(255, 255, 255, 0.3);
  }
}
</style>
