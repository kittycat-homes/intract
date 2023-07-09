<template>
  <div class="filter-box">
    <slot><h1>meow</h1></slot>
  </div>
</template>

<style scoped>
.filter-box {
  padding: var(--pad-size);
  background-color: var(--light-orange);
  width: 100%;
  height: var(--pad-size);
}
@media (orientation: landscape) {
  .filter-box {
    width: fit-content;
    height: 100%;
    order: 500;
  }
}
</style>
