<template>
  <LoadingSpinner v-if="server.loading" />
  <ContentBox v-else-if="!server.error">
    <ContentCard>
      <h1>login to {{ server.info?.name }}</h1>
      <p>
        don't have an account yet?
        <router-link to="register">register</router-link>
      </p>
    </ContentCard>
  </ContentBox>
</template>

<script setup lang="ts">
import LoadingSpinner from "@/components/LoadingSpinner.vue";
import { useServerInfoStore } from "@/store/server_info";
import ContentBox from "@/components/ContentBox.vue";
import ContentCard from "@/components/ContentCard.vue";
const server = useServerInfoStore();
const { fetchServerInfo } = useServerInfoStore();
fetchServerInfo();
</script>

<style scoped></style>
