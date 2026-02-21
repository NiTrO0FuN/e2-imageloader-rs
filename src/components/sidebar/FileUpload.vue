<template>
  <button @click="pickImages">{{ $t("sidebar.add") }}</button>
  <img :src="path" />
</template>

<script lang="ts" setup>
import { Store } from "@tauri-apps/plugin-store";
import { open } from "@tauri-apps/plugin-dialog";
import { ref } from "vue";
let path = ref("");

async function pickImages() {
  let files = await open({
    multiple: true,
    filters: [{ name: "Image", extensions: ["png", "jpeg", "jpg", "webp"] }],
  });

  if (!files) return;
  const store = await Store.load("images.json");
  const old_paths = (await store.get<string[]>("paths")) || [];
  await store.set("paths", old_paths.concat(files));
}
</script>

<style scoped>
button {
  background-color: var(--primary-color);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  border-radius: 5px;
  padding: 5px 15px;
  margin: 5px;
  text-align: center;
}
</style>
