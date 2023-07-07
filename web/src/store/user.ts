import { conf } from "@/api";
import { AccountApi, User } from "@/swagger";
import { defineStore } from "pinia";

type UserData = {
  info: User | null;
  error: boolean;
  loading: boolean;
};

export const useUserStore = defineStore({
  id: "user",
  state: () =>
    ({
      info: null,
      error: false,
      loading: false,
    } as UserData),
  actions: {
    async whoami() {
      this.loading = true;
      await new AccountApi(conf())
        .whoami()
        .then(
          (value) => {
            this.info = value;
          },
          (error) => {
            this.error = true;
          }
        )
        .finally(() => {
          this.loading = false;
        });
    },
  },
});
