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
}

.card {
  &::v-deep p {
    font-size: var(--fs-regular);
  }
  &::v-deep h1 {
    font-size: var(--fs-xl);
  }
}
</style>
