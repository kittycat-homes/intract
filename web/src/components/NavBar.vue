<template>
  <nav>
    <NavButton
      v-for="button in buttons"
      v-bind:key="button.link"
      :link="button.link"
      :name="button.name"
    >
      <template v-slot:icon>
        <RssIcon v-if="button.icon === Icons.Rss"></RssIcon>
        <HomeIcon v-else-if="button.icon === Icons.Home"></HomeIcon>
      </template>
    </NavButton>
  </nav>
</template>

<script setup lang="ts">
import NavButton from "./NavButton.vue";
import { RssIcon, HomeIcon } from "@heroicons/vue/24/solid";
</script>

<script lang="ts">
enum Icons {
  Home,
  Rss,
}

const buttons = [
  { name: "home", link: "/", icon: Icons.Home },
  { name: "about", link: "about", icon: Icons.Rss },
];
</script>

<style scoped>
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
