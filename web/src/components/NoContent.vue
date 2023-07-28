<template>
  <span>
    <FaceFrownIcon />
    <bold>nothing to see here</bold>
  </span>
</template>

<script setup lang="ts">
import { FaceFrownIcon } from "@heroicons/vue/24/solid";
</script>

<style scoped>
span {
  color: var(--light-orange);
}

svg {
  width: var(--fs-xl);
}

span {
  align-items: center;
  justify-content: center;
  display: flex;
  flex-wrap: wrap;
  gap: var(--pad-size);
}

bold {
  font-size: var(--fs-large);
}
</style>
