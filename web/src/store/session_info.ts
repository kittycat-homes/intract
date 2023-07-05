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
      if (this.session != null) {
        return;
      }
      new AccountApi(conf()).login({
        loginData: { password: password, username: username },
      });
    },
  },
});
