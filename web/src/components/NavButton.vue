<template>
  <router-link class="link" :to="link">
    <v-icon class="icon" scale="1.4" :name="icon" />
    <span>{{ name }}</span></router-link
  >
</template>

<script setup lang="ts">
import { toRefs } from "vue";

const props = defineProps<{
  name: string;
  link: string;
  icon: string;
}>();
const { name, link, icon } = toRefs(props);
</script>

<style>
.link {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.icon {
  background-color: var(--black);
  color: var(--light-orange);
}

.router-link-active .icon,
.link:hover > .icon {
  color: var(--black);
  background-color: var(--light-orange);
}
</style>
