<template>
  <div class="card">
    <slot></slot>
  </div>
</template>

<style scoped lang="scss">
.card {
  display: flex;
  flex-direction: column;
  padding: var(--pad-size);
  background-color: var(--light-orange);
  border-radius: var(--radius);
  flex: 1;
  gap: var(--pad-size);
}

.card {
  &:deep p {
    font-size: var(--fs-regular);
  }
  &:deep h1 {
    font-size: var(--fs-xl);
  }
  &:deep * {
    color: var(--black);
  }
}
</style>
