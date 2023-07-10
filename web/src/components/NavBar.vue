<template>
  <nav>
    <NavButton link="/" name="home"><HomeIcon /></NavButton>
    <NavButton v-if="userStore.info" link="/feeds" name="feeds"
      ><RssIcon
    /></NavButton>
    <NavButton
      v-if="userStore.info != null"
      link="/me"
      :name="userStore.info?.username"
      ><UserIcon
    /></NavButton>
    <NavButton v-if="userStore.info === null" link="/login" name="login"
      ><ArrowRightOnRectangleIcon
    /></NavButton>
    <NavButton link="/more" name="more"><EllipsisHorizontalIcon /></NavButton>
  </nav>
</template>

<script setup lang="ts">
import NavButton from "./NavButton.vue";
import {
  HomeIcon,
  ArrowRightOnRectangleIcon,
  UserIcon,
  EllipsisHorizontalIcon,
  RssIcon,
} from "@heroicons/vue/24/solid";
import { useUserStore } from "@/store/user";
const userStore = useUserStore();
if (
  !userStore.loading &&
  userStore.status === null &&
  userStore.info === null
) {
  userStore.whoami();
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
