<template>
  <div class="container">
    <div class="image" v-for="(image, i) in images">
      <img :src="convertFileSrc(image)" height="128" />
      <button @click="process(image)">{{ $t("image.select") }}</button>
      <button @click="remove(i)" class="danger">{{ $t("image.remove") }}</button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, Ref, onMounted } from "vue";
import { Store } from "@tauri-apps/plugin-store";
import { convertFileSrc } from "@tauri-apps/api/core";
import { process } from "../utils/processing";
import { createToast, ToastSeverity } from "../utils/toasts";

let images: Ref<string[]> = ref([]);

let store: Store;
onMounted(async () => {
  store = await Store.load("images.json");
  images.value = (await store.get<string[]>("paths")) || [];
  store.onKeyChange<string[]>("paths", (paths) => (images.value = paths || []));
});

function remove(i: number) {
  images.value.splice(i, 1);
  store.set("paths", images.value);
  createToast("image.remove-success", ToastSeverity.INFO);
}
</script>

<style scoped>
div.container {
  margin-left: 250px;
  padding: 15px;
  display: flex;
  flex-wrap: wrap;
  align-content: start;
  gap: 10px;
  height: calc(100vh - 80px);
  overflow-y: scroll;
  &::-webkit-scrollbar {
    display: none;
  }
}

div.image {
  background-color: var(--primary-color);
  padding: 5px;
  display: block;
  height: fit-content;

  img {
    display: block;
  }

  button {
    background-color: var(--secondary-contrast-color);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    border-radius: 5px;
    width: 100%;
    text-align: center;
    margin-top: 5px;
    display: block;

    &.danger {
      background-color: var(--danger-color);
    }
  }
}
</style>
