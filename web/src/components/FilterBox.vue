<template>
  <div class="filter-box">
    <button class="hide-button" @click="state.show = !state.show">
      <XMarkIcon v-if="state.show" />
      <Bars3Icon v-else />
      {{ state.show ? "close menu" : "open menu" }}
    </button>

    <div class="slot-content" v-if="state.show">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped lang="scss">
.filter-box {
  display: flex;
  background-color: var(--light-orange);
  width: 100%;
  align-items: center;
  justify-content: center;
  flex-direction: column;
}

.slot-content {
  padding: var(--pad-size);
  display: flex;
  flex-direction: row;
  gap: var(--pad-size);
}

.hide-button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--pad-size);
  width: 100%;
  height: 100%;
  background-color: var(--light-orange-clickable);
  border: none;
  cursor: pointer;
  padding: var(--pad-size);
  font-size: var(--fs-regular);
}

.hide-button:hover {
  background-color: var(--light-orange-hover);
}

.filter-box:deep(.link) {
  display: flex;
  flex-direction: row;
  flex-wrap: none;
  align-items: center;
  gap: var(--pad-size);
  background-color: var(--light-orange-clickable);
  padding: var(--pad-size);
  border-radius: var(--radius);
  text-decoration: none;
  color: var(--black);
}

.filter-box:deep(.link:hover) {
  background-color: var(--light-orange-hover);
}

.filter-box:deep(h1) {
  font-size: var(--fs-xl);
}

.filter-box:deep(.link) {
  font-size: var(--fs-regular);
}

.filter-box:deep(.link svg) {
  width: var(--fs-regular);
  height: var(--fs-regular);
}

.filter-box:deep(span) {
  font-size: var(--fs-regulars);
}

svg {
  height: var(--fs-regular);
  width: var(--fs-regular);
}

@media (orientation: landscape) {
  .filter-box {
    height: 100%;
    order: 500;
    flex-direction: row;
    width: min-content;
  }
  .filter-box:deep(.link > span) {
    white-space: nowrap;
  }

  .slot-content {
    display: flex;
    flex-direction: column;
  }

  .hide-button {
    width: min-content;
    flex-direction: column;
  }
}
</style>

<script setup lang="ts">
import { useFilterBarStore } from "@/store/filter_bar";
import { XMarkIcon, Bars3Icon } from "@heroicons/vue/24/solid";
const state = useFilterBarStore();
</script>
