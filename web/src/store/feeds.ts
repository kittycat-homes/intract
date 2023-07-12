import { conf } from "@/api";
import {
  Feed,
  FeedApi,
  FeedsInput,
  ServerApi,
  ServerInfo,
  ShowFeedRequest,
} from "@/swagger";
import { defineStore } from "pinia";

type ServerInfoStoreState = {
  error: boolean;
  loading: boolean;
  show_hidden: boolean;
  info: Feed | null;
};

export const useServerInfoStore = defineStore({
  id: "server-info",
  state: () =>
    ({
      show_hidden: false,
      error: false,
      loading: false,
      info: null,
    } as ServerInfoStoreState),
  actions: {
    async fetchServerInfo() {
      if (this.info != null) {
        return;
      }
      this.loading = true;
      this.info = await new FeedApi(conf())
        .showFeed({
          feedsInput: { showHidden: this.show_hidden } as FeedsInput,
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
