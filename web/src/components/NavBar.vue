<template>
  <div class="navigation">
    <div class="spinner-box" v-if="session.loading || userStore.loading">
      <div class="spinner"></div>
    </div>
    <nav v-else>
      <div class="buttons">
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
        <NavButton link="/more" name="more"
          ><EllipsisHorizontalIcon
        /></NavButton>
      </div>
    </nav>
  </div>
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
import { useSessionInfoStore } from "@/store/session_info";
const userStore = useUserStore();
const session = useSessionInfoStore();
if (
  !userStore.loading &&
  userStore.status === null &&
  userStore.info === null
) {
  userStore.whoami();
}
</script>

<style scoped>
.navigation {
  display: flex;
  flex-direction: row;
  gap: var(--pad-size);
  padding: var(--pad-size);
  justify-content: center;
  align-items: center;
  padding: var(--pad-size);
  background-color: var(--primary-accent);
}

.buttons {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  gap: var(--pad-size);
}

@media (orientation: landscape) {
  nav {
    flex-direction: column;
  }
  .buttons {
    flex-direction: column;
  }
}

.spinner-box {
  height: var(--fs-xxl);
  width: var(--fs-xxl);
}

.spinner {
  border-style: solid;
  border-color: var(--black);
  border-width: var(--fs-regular);
  border-top-color: var(--light-orange);
  border-radius: var(--fs-xxl);
  height: 100%;
  width: 100%;
  animation: spin 6s linear infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg) scale(1);
  }
  50% {
    transform: rotate(1080) scale(2);
  }
  100% {
    transform: rotate(2160deg) scale(1);
  }
}
</style>
