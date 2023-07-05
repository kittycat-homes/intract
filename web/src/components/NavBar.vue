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
        <ArrowRightOnRectangleIcon
          v-else-if="button.icon === Icons.Login"
        ></ArrowRightOnRectangleIcon>
      </template>
    </NavButton>
  </nav>
</template>

<script setup lang="ts">
import NavButton from "./NavButton.vue";
import {
  RssIcon,
  HomeIcon,
  ArrowRightOnRectangleIcon,
} from "@heroicons/vue/24/solid";
</script>

<script lang="ts">
enum Icons {
  Home,
  Rss,
  Login,
}

const buttons = [
  { name: "home", link: "/", icon: Icons.Home },
  { name: "about", link: "about", icon: Icons.Rss },
  { name: "login", link: "login", icon: Icons.Login },
];
</script>

<style scoped>
nav {
  display: flex;
  flex-direction: row;
  gap: var(--pad-size);
  padding: var(--pad-size);
}

@media (orientation: landscape) {
  nav {
    flex-direction: column;
  }
}
</style>
