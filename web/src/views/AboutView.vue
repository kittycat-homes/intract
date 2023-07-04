<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useServerInfoStore } from "@/store/server_info";
const { info, loading } = storeToRefs(useServerInfoStore());
const { fetchServerInfo } = useServerInfoStore();
fetchServerInfo();
</script>

<template>
  <div class="about">
    <h1 v-if="loading">loading</h1>
    <h1 v-else>{{ info }}</h1>
  </div>
</template>
