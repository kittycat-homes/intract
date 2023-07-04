<script setup lang="ts">
import NavBar from "@/components/NavBar.vue";
</script>

<template>
  <div id="app">
    <NavBar id="nav-bar" />
    <router-view id="router-view" />
  </div>
</template>

<style lang="scss">
#app {
  height: 100vh;
  width: 100vw;
  overflow-y: clip;
  display: flex;
  flex-direction: column-reverse;
}

@media (orientation: landscape) {
  #app {
    flex-direction: row;
  }
}

#router-view {
  flex-grow: 1;
  background-color: var(--black);
  overflow-y: scroll;
}

nav {
  background-color: var(--primary-accent);
}
</style>
