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

.link::v-deep svg {
  height: 2.4em;
  width: 2.4em;
  background-color: var(--black);
  color: var(--light-orange);
}

.router-link-active::v-deep > svg,
.link:hover::v-deep > svg {
  color: var(--black);
  background-color: var(--light-orange);
}

.text {
  color: var(--black);
}
</style>
