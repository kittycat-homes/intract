import { conf } from "@/api";
import { ServerApi, ServerInfo } from "@/swagger";
import { defineStore } from "pinia";

type ServerInfoStoreState = {
  error: boolean;
  loading: boolean;
  info: ServerInfo | null;
};

export const useServerInfoStore = defineStore({
  id: "server-info",
  state: () =>
    ({
      error: false,
      loading: true,
      info: null,
    } as ServerInfoStoreState),
  actions: {
    async fetchServerInfo() {
      if (this.info != null) {
        return;
      }
      this.info = await new ServerApi(conf())
        .serverInfo()
        .catch((error) => {
          console.log(error);
          this.error = true;
          return null;
        })
        .finally(() => {
          this.loading = false;
        });
    },
  },
});
