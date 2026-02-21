<template>
  <NavBar />
  <SideBar />
  <ImageViewer />
  <Toasts />
</template>
<script setup lang="ts">
import Toasts from "./components/Toasts.vue";
import ImageViewer from "./components/ImageViewer.vue";
import NavBar from "./components/navbar/NavBar.vue";
import SideBar from "./components/sidebar/SideBar.vue";
</script>

<style scoped></style>
