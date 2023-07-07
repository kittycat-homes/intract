<template>
  <div class="input-wrap">
    <label for="id">
      {{ label }}
    </label>
    <input
      class="in"
      :id="id"
      :type="secret_actual_type"
      :value="modelValue"
      :placeholder="placeholder"
      @input="
        emit('update:modelValue', ($event.target as HTMLInputElement).value)
      "
    />
    <button v-if="type === 'password'" @click.prevent="toggle_password">
      {{ secret_actual_type === "password" ? "show" : "hide" }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "@vue/runtime-dom";

const props = defineProps({
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

const secret_actual_type = ref(props.type);

const toggle_password = () => {
  if (secret_actual_type.value === "text") {
    secret_actual_type.value = "password";
  } else if (secret_actual_type.value === "password") {
    secret_actual_type.value = "text";
  }
};
</script>

<style scoped>
.input-wrap {
  display: flex;
  flex-direction: column;
  border-style: solid;
  border-width: var(--border-width);
  border-color: var(--secondary-accent);
  border-radius: calc(var(--radius) - var(--pad-size));
  align-items: stretch;
  background-color: var(--secondary-accent);
}

@media (orientation: landscape) {
  .input-wrap {
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: var(--pad-size);
  }
}

.input-wrap:focus-within {
  border-color: var(--primary-accent);
  background-color: var(--primary-accent);
}

input {
  padding: var(--pad-size);
  background-color: var(--light-orange-clickable);
  border-radius: calc((var(--radius) - var(--pad-size)) - var(--border-width));
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
  margin: var(--pad-size);
  color: var(--black);
  font-size: var(--fs-regular);
  margin-left: var(--pad-size);
  margin-top: var(--pad-size);
}
</style>
