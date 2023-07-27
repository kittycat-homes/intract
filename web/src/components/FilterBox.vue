<template>
  <div class="filter-box">
    <slot></slot>
  </div>
</template>

<style scoped lang="scss">
.filter-box {
  display: flex;
  padding: var(--pad-size);
  background-color: var(--light-orange);
  width: 100%;
  align-items: center;
  justify-content: center;
}
@media (orientation: landscape) {
  .filter-box {
    width: fit-content;
    height: 100%;
    order: 500;
  }
  .filter-box:deep(.link > span) {
    white-space: nowrap;
  }
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

.filter-box:deep(.link svg) {
  width: var(--fs-regular);
  height: var(--fs-regular);
}
</style>
