import { conf } from "@/api";
import { AccountApi, ResponseError } from "@/swagger";
import { defineStore } from "pinia";

type SessionInfoState = {
  status: number | null;
  loading: boolean;
};

export const useSessionInfoStore = defineStore({
  id: "session-info",
  state: () =>
    ({
      status: null,
      loading: false,
      session: null,
    } as SessionInfoState),
  actions: {
    async login(password: string, username: string) {
      this.loading = true;
      try {
        await new AccountApi(conf()).loginRaw({
          loginData: { password: password, username: username },
        });
        this.status = 200;
      } catch (e) {
        if (e instanceof ResponseError) {
          this.status = e.response.status;
        } else {
          this.status = 500;
        }
        console.log(e);
      } finally {
        this.loading = false;
      }
    },
  },
});
