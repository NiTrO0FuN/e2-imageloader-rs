<template>
  <select class="locale-changer" v-model="$i18n.locale">
    <option v-for="locale in $i18n.availableLocales" :key="`locale-${locale}`" :value="locale">{{ locale }}</option>
  </select>
</template>

<script lang="ts" setup>
import { Store } from "@tauri-apps/plugin-store";
import { watch } from "vue";
import { useI18n } from "vue-i18n";

const i18n = useI18n();

watch(i18n.locale, async (newLocale) => {
  const store = await Store.load("settings.json");
  await store.set("lang", newLocale);
});
</script>

<style>
.locale-changer {
  margin-right: 15px;
  padding: 5px;
  background-color: var(--primary-color);
  font-size: large;
  color: white;
  border-radius: 5px;

  option:checked {
    font-weight: bold;
  }
}
</style>
