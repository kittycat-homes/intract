<template>
  <div id="box">
    <slot></slot>
  </div>
</template>

<style scoped>
#box {
  display: flex;
  flex-wrap: wrap;
  gap: var(--pad-size);
  padding: var(--pad-size);
}
</style>
