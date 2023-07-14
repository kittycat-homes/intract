<template>
  <div class="filter-box">
    <slot></slot>
  </div>
</template>

<style scoped>
.filter-box {
  display: flex;
  padding: var(--pad-size);
  background-color: var(--light-orange);
  width: 100%;
  height: var(--pad-size);
  align-items: center;
  flex-shrink: 1;
}
@media (orientation: landscape) {
  .filter-box {
    width: fit-content;
    height: 100%;
    order: 500;
  }
}

.filter-box:deep(.link) {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--pad-size);
  background-color: var(--light-orange-clickable);
  padding: var(--pad-size);
  border-radius: var(--radius);
}

.filter-box:deep(.link svg) {
  width: var(--fs-regular);
  height: var(--fs-regular);
}
</style>
