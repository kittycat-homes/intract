<template>
  <FilterBox>
    <router-link class="link" to="/feeds/follow"
      ><BookmarkIcon /><span>follow feed</span></router-link
    >
  </FilterBox>
  <LoadingSpinner v-if="feed_store.loading" />
  <ContentBox v-else>
    <NoContent v-if="feed_store.items.length === 0" />
    <ContentCard v-for="item in feed_store.items" :key="item.url">
      <template v-slot:title>
        <h2>{{ item.title ?? item.url }}</h2></template
      ></ContentCard
    >
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
import { useFeedStore } from "../store/feeds";
import LoadingSpinner from "@/components/LoadingSpinner.vue";
import NoContent from "@/components/NoContent.vue";

const feed_store = useFeedStore();
feed_store.fetchServerInfo();
</script>
