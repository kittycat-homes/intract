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
import router from "@/router";
import { useUserStore } from "@/store/user";

// stores
const server = useServerInfoStore();
const session = useSessionInfoStore();
const user = useUserStore();
const { fetchServerInfo } = useServerInfoStore();
const { login } = useSessionInfoStore();
const { whoami } = useUserStore();

// get info
fetchServerInfo();

// form things
const username = ref("");
const password = ref("");

const sendLogin = () => {
  login(password.value, username.value);
};

const leave_page_cuz_logged_in = () => {
  if (session.session != null && user.info != null) {
    router.push("/");
  }
};

leave_page_cuz_logged_in();

session.$subscribe((_mutation, _state) => {
  whoami();
  leave_page_cuz_logged_in();
});

user.$subscribe((_mutation, _state) => {
  leave_page_cuz_logged_in();
});
</script>

<style scoped>
form {
  display: flex;
  flex-direction: column;
  gap: var(--pad-size);
}
</style>
