<template>
  <ContentBox>
    <ContentCard>
      <template v-slot:title
        ><h1>follow</h1>
        <p>add an rss, atom or json feed you like</p></template
      >
    </ContentCard>
  </ContentBox>
</template>

<script setup lang="ts">
import ContentBox from "@/components/ContentBox.vue";
import ContentCard from "@/components/ContentCard.vue";
</script>
