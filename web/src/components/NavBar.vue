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
        <UserIcon v-else-if="button.icon === Icons.User" />
      </template>
    </NavButton>
  </nav>
</template>

<script setup lang="ts">
import { Ref, ref } from "vue";
import NavButton from "./NavButton.vue";
import {
  RssIcon,
  HomeIcon,
  ArrowRightOnRectangleIcon,
  UserIcon,
} from "@heroicons/vue/24/solid";
import { useSessionInfoStore } from "@/store/session_info";
import { useUserStore } from "@/store/user";

const buttons: Ref<Button[]> = ref([]);

const sessionStore = useSessionInfoStore();
const userStore = useUserStore();

const generate_buttons = () => {
  buttons.value = [
    { name: "home", link: "/", icon: Icons.Home },
    { name: "about", link: "/about", icon: Icons.Rss },
    { name: "login", link: "/login", icon: Icons.Login },
    {
      name: userStore.info?.displayName ?? userStore.info?.username,
      link: "/me",
      icon: Icons.User,
    },
  ].filter((value) => {
    // dont show login if user is already logged in
    if (value.link === "/login" && sessionStore.session != null) {
      return false;
    }
    if (value.link === "/me" && userStore.info == null) {
      if (!userStore.loading && !userStore.error) {
        userStore.whoami();
      }
      return false;
    }
    return true;
  });
};

// regenerate to change buttons when user is logged in
sessionStore.$subscribe((_mutation, _state) => {
  generate_buttons();
});
userStore.$subscribe((_mutation, _state) => {
  generate_buttons();
});

generate_buttons();
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
