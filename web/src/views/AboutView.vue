<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useServerInfoStore } from "@/store/server_info";
const { info, loading } = storeToRefs(useServerInfoStore());
const { fetchServerInfo } = useServerInfoStore();
fetchServerInfo();
</script>

<template>
  <div class="about">
    <h1>about</h1>
    <h1 v-if="loading">loading</h1>
    <h1 v-else>{{ info }}</h1>
  </div>
</template>

<style scoped>
h1 {
  font-size: var(--fs-large);
}

.about {
  background-color: var(--light-orange);
}
</style>
