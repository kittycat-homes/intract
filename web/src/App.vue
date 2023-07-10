<script setup lang="ts">
import NavBar from "@/components/NavBar.vue";
</script>

<template>
  <div id="app">
    <NavBar />
    <div class="router-view">
      <router-view />
    </div>
  </div>
</template>

<style scoped>
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

@media (orientation: landscape) {
  #app {
    flex-direction: row;
  }

  #app:deep(.router-view) {
    flex-direction: row;
  }
}
</style>
