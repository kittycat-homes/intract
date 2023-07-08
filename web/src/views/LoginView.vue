<template>
  <LoadingSpinner v-if="server.loading || session.loading" />
  <ContentBox v-else-if="!server.error && server.info">
    <ContentCard>
      <template v-slot:title>
        <h1>login to {{ server.info?.name }}<br /></h1>
        <p>
          don't have an account yet?
          <router-link to="register">register</router-link>!
        </p>
      </template>

      <form @submit.prevent="send_login">
        <BaseInput
          id="login-username"
          label="username"
          placeholder="xxXCoolGamer42Xxx"
          v-model="username"
        />
        <ErrorNotice
          v-if="
            username.length < server.info.minPasswordLength &&
            username.length > 0
          "
          >username too short</ErrorNotice
        >

        <BaseInput
          id="login-pass"
          type="password"
          label="password"
          placeholder="hunter2"
          v-model="password"
        />
        <ErrorNotice
          v-if="
            password.length < server.info.minPasswordLength &&
            password.length > 0
          "
          >password too short</ErrorNotice
        >
        <PrimaryButton><ArrowLeftOnRectangleIcon />login</PrimaryButton>
      </form>
      <div v-if="session.status != null">
        <ErrorNotice>{{ generate_error_message() }}</ErrorNotice>
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
import PrimaryButton from "@/components/PrimaryButton.vue";
import { ArrowLeftOnRectangleIcon } from "@heroicons/vue/24/solid";

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

const send_login = () => {
  login(password.value, username.value);
};

const leave_page_cuz_logged_in = () => {
  if (session.session != null && user.info != null) {
    router.push("/");
  }
};

leave_page_cuz_logged_in();

session.$subscribe(() => {
  whoami();
  leave_page_cuz_logged_in();
});

user.$subscribe(() => {
  leave_page_cuz_logged_in();
});

const generate_error_message = (): string => {
  if (session.status === 404) {
    return "nobody with this username seems to exist...";
  }

  if (session.status === 418) {
    return "your account is still waiting for approval, come back later";
  }

  if (session.status === 401) {
    return "this is the wrong password";
  }

  return "failed to log in";
};
</script>

<style scoped>
form {
  display: flex;
  flex-direction: column;
  gap: var(--pad-size);
}
</style>
