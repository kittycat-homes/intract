<template>
  <ContentBox>
    <ContentCard>
      <h2>settings for {{ userStore.info?.username }}</h2>
      <button @click="logout">logout</button>
    </ContentCard>
  </ContentBox>
</template>

<script setup lang="ts">
import ContentBox from "@/components/ContentBox.vue";
import ContentCard from "@/components/ContentCard.vue";
import router from "@/router";
import { useUserStore } from "@/store/user";

const userStore = useUserStore();
if (userStore.info === null) {
  router.push("/login");
}
</script>

<script lang="ts">
const logout = () => {
  localStorage.clear();
  location.reload();
};
</script>
