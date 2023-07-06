<script setup lang="ts">
import NavBar from "@/components/NavBar.vue";
import { useUserStore } from "./store/user";
import { useSessionInfoStore } from "./store/session_info";
import LoadingSpinner from "./components/LoadingSpinner.vue";

let userStore = useUserStore();
let sessionStore = useSessionInfoStore();
</script>

<template>
  <div id="app">
    <NavBar id="nav-bar" />
    <div class="router-view">
      <router-view />
    </div>
  </div>
</template>

<style>
#app {
  height: 100vh;
  width: 100vw;
  overflow-y: hidden;
  overflow-x: hidden;
  display: flex;
  flex-direction: column-reverse;
  background-color: var(--black);
}

@media (orientation: landscape) {
  #app {
    flex-direction: row;
  }
}

.router-view {
  background-color: var(--black);
  overflow-y: scroll;
  overflow-x: hidden;
  flex-grow: 1;
  flex-direction: column;
}

nav {
  background-color: var(--primary-accent);
}
</style>
