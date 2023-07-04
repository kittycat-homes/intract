<template>
  <nav>
    <NavButton
      v-for="button in buttons"
      v-bind:key="button.link"
      :link="button.link"
      :name="button.name"
      :icon="button.icon"
    />
  </nav>
</template>

<script setup lang="ts">
import NavButton from "./NavButton.vue";
</script>

<script lang="ts">
const buttons = [
  { name: "home", link: "/", icon: "hi-home" },
  { name: "about", link: "about", icon: "hi-rss" },
];
</script>

<style>
nav {
  display: flex;
  flex-direction: row;
}

@media (orientation: landscape) {
  nav {
    flex-direction: column;
  }
}
</style>
