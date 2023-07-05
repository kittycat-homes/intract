<template>
  <div class="error-notice">
    <ExclamationTriangleIcon class="icon" />
    <p class="description">
      <slot>meow</slot>
    </p>
  </div>
</template>

<script setup lang="ts">
import { ExclamationTriangleIcon } from "@heroicons/vue/24/solid";
</script>

<style scoped>
.description {
  font-size: var(--fs-regular);
}
.error-notice {
  display: flex;
  align-items: center;
  padding: var(--pad-size);
  background-color: var(--light-red);
  border-radius: calc(var(--radius) - var(--pad-size));
}
.icon {
  float: left;
  width: var(--fs-large);
  height: var(--fs-large);
}
</style>
