<template>
  <div class="box">
    <slot></slot>
  </div>
</template>

<style scoped>
.box {
  display: flex;
  flex-direction: column;
  gap: var(--pad-size);
  padding: var(--pad-size);
  overflow: scroll;
  flex-grow: 1;
  container-type: inline-size;
}
</style>
