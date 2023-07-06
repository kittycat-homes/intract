<template>
  <nav>
    <NavButton link="/" name="home"><HomeIcon /></NavButton>
    <NavButton
      v-if="userStore.info != null"
      link="/me"
      :name="userStore.info?.username"
      ><UserIcon
    /></NavButton>
    <NavButton v-if="sessionStore.session === null" link="/login" name="login"
      ><ArrowRightOnRectangleIcon
    /></NavButton>
  </nav>
</template>

<script setup lang="ts">
import NavButton from "./NavButton.vue";
import {
  HomeIcon,
  ArrowRightOnRectangleIcon,
  UserIcon,
} from "@heroicons/vue/24/solid";
import { useSessionInfoStore } from "@/store/session_info";
import { useUserStore } from "@/store/user";
const sessionStore = useSessionInfoStore();
const userStore = useUserStore();
if (!userStore.loading && !userStore.error && userStore.info === null) {
  userStore.whoami();
}
</script>

<script lang="ts">
type Button = {
  name: string;
  link: string;
  icon: Icons;
};

enum Icons {
  Home,
  Rss,
  Login,
  User,
}
</script>

<style scoped>
nav {
  display: flex;
  flex-direction: row;
  gap: var(--pad-size);
  padding: var(--pad-size);
  justify-content: center;
}

@media (orientation: landscape) {
  nav {
    flex-direction: column;
  }
}
</style>
