import { conf } from "@/api";
import { AccountApi, Session } from "@/swagger";
import { defineStore } from "pinia";

type SessionInfoState = {
  error: boolean;
  loading: boolean;
  session: Session | null;
};

export const useSessionInfoStore = defineStore({
  id: "session-info",
  state: () =>
    ({
      error: false,
      loading: false,
      session: null,
    } as SessionInfoState),
  actions: {
    async login(password: string, username: string) {
      this.loading = true;
      await new AccountApi(conf())
        .login({
          loginData: { password: password, username: username },
        })
        .then((value) => {
          this.error = false;
          this.session = value;
        })
        .catch(() => {
          this.error = true;
          return null;
        })
        .finally(() => {
          this.loading = false;
        });
    },
  },
});
