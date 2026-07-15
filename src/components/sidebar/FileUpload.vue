<template>
  <button @click="pickImages">{{ $t("sidebar.add") }}</button>
</template>

<script lang="ts" setup>
import { Store } from "@tauri-apps/plugin-store";
import { open } from "@tauri-apps/plugin-dialog";

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

<style scoped></style>
