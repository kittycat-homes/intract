<template>
  <router-link class="link" :to="link">
    <slot name="icon"></slot>
    <span class="text">{{ name }}</span></router-link
  >
</template>

<script setup lang="ts">
import { toRefs } from "vue";

const props = defineProps<{
  name: string;
  link: string;
}>();
const { name, link } = toRefs(props);
</script>

<style scoped>
.link {
  text-decoration: none;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.link:deep svg {
  display: flex;
  padding: var(--pad-size-small);
  height: var(--fs-large);
  width: var(--fs-large);
  background-color: var(--black);
  color: var(--light-orange);
  border-radius: var(--fs-large);
}

.router-link-active:deep > svg,
.link:hover:deep > svg {
  color: var(--black);
  background-color: var(--light-orange);
}

.text {
  color: var(--black);
  font-size: var(--fs-small);
}
</style>
