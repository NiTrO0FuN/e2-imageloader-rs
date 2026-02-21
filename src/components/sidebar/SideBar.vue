<template>
  <div id="sidebar">
    <OptionSelector />
    <FileUpload />
  </div>
</template>

<script lang="ts" setup>
import FileUpload from "./FileUpload.vue";
import OptionSelector from "./OptionSelector.vue";
</script>

<style>
#sidebar {
  width: 250px;
  height: 100;
  display: flex;
  flex-direction: column;
  margin: 10px 0px;
  position: fixed;
  left: 0;
}
</style>
