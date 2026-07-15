<template>
  <div id="sidebar">
    <OptionSelector />
    <FileUpload />
    <AutoMode />
  </div>
</template>

<script lang="ts" setup>
import AutoMode from "./AutoMode.vue";
import FileUpload from "./FileUpload.vue";
import OptionSelector from "./OptionSelector.vue";
</script>

<style scoped>
#sidebar {
  width: 250px;
  height: 100;
  display: flex;
  flex-direction: column;
  margin: 10px 0px;
  position: fixed;
  left: 0;
}

button {
  background-color: var(--primary-color);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  border-radius: 5px;
  padding: 5px 15px;
  margin: 5px;
  text-align: center;
}
</style>
