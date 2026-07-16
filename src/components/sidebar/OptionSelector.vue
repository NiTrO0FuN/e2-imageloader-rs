<template>
  <div class="options-container">
    <h2>{{ $t("options.quality.header") }}</h2>
    <label v-for="q in qualities">
      <input type="radio" name="quality" :value="q.value" v-model="options.quality" />
      <div class="label-text">
        <span>{{ $t(q.name) }}</span>
        <span>{{ $t(q.pixels) }}</span>
      </div>
    </label>
    <Separator />
    <h2>{{ $t("options.size.header") }}</h2>
    <label v-for="s in file_sizes">
      <input type="radio" name="size" :value="s.value" v-model="options.chunkSize" />
      {{ $t(s.name) }}
    </label>
  </div>
</template>

<script lang="ts" setup>
import Separator from "../Separator.vue";
import { Quality, ChunkSize, options } from "../../store/options";

const qualities = [
  { name: "options.quality.high.name", pixels: "options.quality.high.pixels", value: Quality.HIGH },
  { name: "options.quality.medium.name", pixels: "options.quality.medium.pixels", value: Quality.MEDIUM },
  { name: "options.quality.low.name", pixels: "options.quality.low.pixels", value: Quality.LOW },
];

const file_sizes = [
  { name: "options.size.large", value: ChunkSize.LARGE },
  { name: "options.size.medium", value: ChunkSize.MEDIUM },
  { name: "options.size.small", value: ChunkSize.SMALL },
];
</script>

<style>
.options-container {
  background-color: var(--primary-color);
  margin: 5px;
  border-radius: 10px;

  h2 {
    text-align: center;
    margin: 10px 0;
  }

  label {
    display: flex;
    margin: 5px;

    .label-text {
      display: flex;
      justify-content: space-between;
      width: 100%;
      padding-right: 5px;
    }
  }
}
</style>
