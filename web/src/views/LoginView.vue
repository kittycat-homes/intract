<template>
  <LoadingSpinner v-if="server.loading || session.loading" />
  <ContentBox v-else-if="!server.error && server.info">
    <ContentCard>
      <h1>login to {{ server.info?.name }}</h1>
      <p>
        don't have an account yet?
        <router-link to="register">register</router-link>
      </p>
      <form @submit.prevent="sendLogin">
        <BaseInput
          id="login-username"
          label="username"
          placeholder="xxXCoolGamer42Xxx"
          v-model="username"
        />
        <BaseInput
          id="login-pass"
          type="password"
          label="password"
          placeholder="hunter2"
          v-model="password"
        />
        <button>login</button>
      </form>
      <div v-if="session.error">
        <ErrorNotice>failed to log in</ErrorNotice>
      </div>
    </ContentCard>
  </ContentBox>
</template>

<script setup lang="ts">
import LoadingSpinner from "@/components/LoadingSpinner.vue";
import { useServerInfoStore } from "@/store/server_info";
import ContentBox from "@/components/ContentBox.vue";
import ContentCard from "@/components/ContentCard.vue";
import { ref } from "@vue/runtime-dom";
import BaseInput from "@/components/BaseInput.vue";
import { useSessionInfoStore } from "@/store/session_info";
import ErrorNotice from "@/components/ErrorNotice.vue";

// stores
const server = useServerInfoStore();
const session = useSessionInfoStore();
const { fetchServerInfo } = useServerInfoStore();
const { login } = useSessionInfoStore();

// get info
fetchServerInfo();

// form things
const username = ref("");
const password = ref("");

const sendLogin = () => {
  login(password.value, username.value);
};
</script>

<style scoped>
form {
  display: flex;
  flex-direction: column;
  gap: var(--pad-size);
}
</style>
