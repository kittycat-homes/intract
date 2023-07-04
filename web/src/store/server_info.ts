import { conf } from "@/api";
import { ServerApi, ServerInfo } from "@/swagger";
import { defineStore } from "pinia";

type ServerInfoStoreState = {
  loading: boolean;
  info: ServerInfo | null;
};

export const useServerInfoStore = defineStore({
  id: "server-info",
  state: () =>
    ({
      loading: true,
      info: null,
    } as ServerInfoStoreState),
  actions: {
    async fetchServerInfo() {
      if (this.info != null) {
        return;
      }
      this.info = await new ServerApi(conf).serverInfo().finally(() => {
        this.loading = false;
      });
    },
  },
});
