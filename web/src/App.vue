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
  overflow-y: scroll;
  display: flex;
  flex-direction: column;
}

@media (orientation: landscape) {
  #app {
    flex-direction: row;
  }
}

#router-view {
  flex-grow: 1;
  background-color: var(--black);
  overflow: scroll;
}

#nav-bar {
  flex-shrink: 100;
}

nav {
  background-color: var(--primary-accent);
  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
