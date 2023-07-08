<template>
  <div>
    <button @click="click()">
      <slot></slot>
    </button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "PrimaryButton",
  emits: ["clicked"],
  methods: {
    click() {
      this.$emit("clicked");
    },
  },
});
</script>

<style scoped>
button {
  background-color: var(--secondary-accent);
  border: none;
  padding: var(--pad-size);
  font-size: var(--fs-regular);
  border-radius: calc(var(--radius) - var(--pad-size));
  cursor: pointer;
  display: flex;
  gap: var(--pad-size);
  align-items: center;
}

button:deep(svg) {
  width: var(--fs-regular);
  height: var(--fs-regular);
}

button:focus,
button:hover {
  background-color: var(--primary-accent);
}

div {
  display: flex;
  align-items: right;
}
</style>
