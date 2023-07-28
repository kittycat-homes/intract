<template>
  <FilterBox>
    <router-link class="link" to="/feeds"
      ><ArrowUturnLeftIcon /><span>back to feeds</span></router-link
    >
  </FilterBox>

  <ContentBox>
    <ContentCard>
      <template v-slot:title
        ><h2>new feed</h2>
        <p>add an rss, atom or json feed you HATE!!!</p></template
      >
    </ContentCard>
    <ContentCard>
      <template v-slot:title
        ><h2>new feed</h2>
        <p>add an rss, atom or json feed you like</p></template
      >
    </ContentCard>
    <ContentCard>
      <template v-slot:title
        ><h2>new feed</h2>
        <p>add an rss, atom or json feed you like</p></template
      >
    </ContentCard>
  </ContentBox>
</template>

<script setup lang="ts">
import ContentBox from "@/components/ContentBox.vue";
import ContentCard from "@/components/ContentCard.vue";
import FilterBox from "@/components/FilterBox.vue";
import { ArrowUturnLeftIcon } from "@heroicons/vue/24/solid";
</script>
