<template>
  <ContentBox>
    <ContentCard>
      <h1>hi! {{ userStore.info?.displayName ?? userStore.info?.username }}</h1>
    </ContentCard>
  </ContentBox>
</template>

<script setup lang="ts">
import ContentBox from "@/components/ContentBox.vue";
import ContentCard from "@/components/ContentCard.vue";
import { useUserStore } from "@/store/user";

const userStore = useUserStore();
</script>
