<template>
  <div class="input-wrap">
    <label for="id">
      {{ label }}
    </label>
    <input
      class="in"
      :id="id"
      :type="type"
      :value="modelValue"
      :placeholder="placeholder"
      @input="
        emit('update:modelValue', ($event.target as HTMLInputElement).value)
      "
    />
  </div>
</template>

<script setup lang="ts">
defineProps({
  label: {
    type: String,
    required: true,
  },
  placeholder: {
    type: String,
    required: true,
  },
  type: {
    type: String,
    default: "text",
  },
  id: {
    type: String,
    required: true,
  },
  modelValue: {
    type: String,
    default: "",
  },
});
const emit = defineEmits<{
  (e: "update:modelValue", payload: string): void;
}>();
</script>

<style scoped>
.input-wrap {
  display: flex;
  flex-direction: column;
  border-style: solid;
  border-color: var(--secondary-accent);
  border-radius: calc(var(--radius) - var(--pad-size));
  align-items: stretch;
}

@media (orientation: landscape) {
  .input-wrap {
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: var(--pad-size);
  }

  label {
    margin-top: 0 !important;
  }
}

.input-wrap:focus-within {
  border-color: var(--primary-accent);
}

input {
  padding: var(--pad-size);
  background-color: var(--light-orange-clickable);
  border-top-right-radius: calc(var(--radius) - var(--pad-size));
  border-bottom-right-radius: calc(var(--radius) - var(--pad-size));
  border-style: none;
  font-size: var(--fs-regular);
  flex-grow: 1;
  color: var(--black);
}

input::placeholder {
  color: var(--secondary-accent);
  opacity: 100%;
}

input:focus::placeholder {
  color: var(--primary-accent);
}

input:focus {
  outline: none;
}

label {
  padding: auto;
  color: var(--black);
  font-size: var(--fs-small);
  margin-left: var(--pad-size);
  margin-top: var(--pad-size);
}
</style>
