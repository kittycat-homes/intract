<template>
  <div class="input-wrap">
    <label for="id">
      {{ label }}
    </label>
    <div class="button-and-input">
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
      <button
        class="show-password-button"
        v-if="type === 'password'"
        @click.prevent="toggle_password"
      >
        <EyeIcon v-if="secret_actual_type === 'password'" class="icon" />
        <EyeSlashIcon v-else class="icon" />
        {{ secret_actual_type === "password" ? "show" : "hide" }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "@vue/runtime-dom";
import { EyeSlashIcon, EyeIcon } from "@heroicons/vue/24/solid";

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
  color: var(--black);
  flex-grow: 1;
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

.show-password-button {
  background-color: var(--light-orange-clickable);
  border: none;
  padding: var(--pad-size);
  border-radius: calc((var(--radius) - var(--pad-size)) - var(--border-width));
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--pad-size);
  font-size: var(--fs-small);
}

.icon {
  width: var(--fs-regular);
  height: var(--fs-regular);
}

.show-password-button:focus,
.show-password-button:hover {
  background-color: var(--light-orange-hover);
  cursor: pointer;
}

.button-and-input {
  display: flex;
  flex-direction: row;
  flex-grow: 1;
  gap: var(--border-width);
}
</style>
