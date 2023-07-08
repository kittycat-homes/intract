import { conf } from "@/api";
import { AccountApi, ResponseError, User } from "@/swagger";
import { defineStore } from "pinia";

type UserData = {
  info: User | null;
  status: null | number;
  loading: boolean;
};

export const useUserStore = defineStore({
  id: "user",
  state: () =>
    ({
      info: null,
      status: null,
      loading: false,
    } as UserData),
  actions: {
    async whoami() {
      this.loading = true;
      try {
        const response = await new AccountApi(conf()).whoamiRaw();
        this.info = await response.value();
      } catch (e) {
        if (e instanceof ResponseError) {
          this.status = e.response.status;
        }
        this.status = 500;
      } finally {
        this.loading = false;
      }
    },
  },
});
