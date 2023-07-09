<script setup lang="ts">
import NavBar from "@/components/NavBar.vue";
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

.router-view {
  display: flex;
  background-color: var(--black);
  overflow: hidden;
  flex-grow: 1;
  flex-direction: column;
}

nav {
  background-color: var(--primary-accent);
}

@media (orientation: landscape) {
  #app {
    flex-direction: row;
  }

  .router-view {
    flex-direction: row;
  }
}
</style>
