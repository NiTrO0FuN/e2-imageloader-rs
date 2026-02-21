<template>
  <nav class="navbar">
    <div style="flex-grow: 1"></div>
    <LocaleSelector />
    <GamePath />
  </nav>
</template>

<script lang="ts" setup>
import GamePath from "./GamePath.vue";
import LocaleSelector from "./LocaleSelector.vue";
</script>

<style>
.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 50px;
  box-shadow: 0 3px 10px rgba(0, 0, 0, 0.5);
}
</style>
