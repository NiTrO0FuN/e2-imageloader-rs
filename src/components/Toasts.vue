<template>
  <TransitionGroup class="toasts-container" name="toast" tag="li">
    <ul v-for="(toast, i) in toasts" :key="toast.timestamp" :class="['toast', toast.severity]" @click="removeToast(i)">
      {{
        $t(toast.text)
      }}
    </ul>
  </TransitionGroup>
</template>

<script lang="ts" setup>
import { toasts, removeToast } from "../utils/toasts";
</script>

<style>
.toasts-container {
  right: 0;
  bottom: 0;
  height: 100%;
  position: absolute;
  overflow: hidden;
  padding: 10px;
  display: flex;
  flex-direction: column;
  justify-content: end;
  pointer-events: none;

  .toast {
    pointer-events: auto;
    margin-top: 10px;
    margin-bottom: 0px;
    padding: 5px 10px;
    border-radius: 5px;
    box-shadow: 0 4px 4px rgba(0, 0, 0, 0.2);

    &.info {
      background-color: var(--primary-color);
    }
    &.success {
      background-color: var(--positive-color);
    }
    &.alert {
      background-color: var(--danger-color);
    }
  }
}

.toast-enter-from {
  opacity: 0;
  transform: translateY(30px);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

.toast-move,
.toast-enter-active,
.toast-leave-active {
  transition: all 0.5s ease;
}
</style>
