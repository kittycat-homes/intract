<template>
  <FilterBox>
    <router-link to="/feeds/follow"><BookmarkIcon />e</router-link>
  </FilterBox>
  <ContentBox>
    <ContentCard></ContentCard>
    <ContentCard></ContentCard>
    <ContentCard></ContentCard>
    <ContentCard></ContentCard>
    <ContentCard></ContentCard>
  </ContentBox>
</template>

<style>
.action-button {
  position: relative;
  right: 0em;
  bottom: 0em;
}
</style>

<script setup lang="ts">
import ContentBox from "@/components/ContentBox.vue";
import FilterBox from "@/components/FilterBox.vue";
import ContentCard from "@/components/ContentCard.vue";
import { BookmarkIcon } from "@heroicons/vue/24/solid";
</script>
