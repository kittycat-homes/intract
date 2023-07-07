import { conf } from "@/api";
import router from "@/router";
import { AccountApi, ResponseError, Session } from "@/swagger";
import { defineStore } from "pinia";

type SessionInfoState = {
  status: number | null;
  loading: boolean;
  session: Session | null;
};

export const useSessionInfoStore = defineStore({
  id: "session-info",
  persist: true,
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
        const data = await new AccountApi(conf()).loginRaw({
          loginData: { password: password, username: username },
        });
        this.session = await data.value();
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
