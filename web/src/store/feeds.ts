import { conf } from "@/api";
import { Feed, FeedApi, FeedsInput } from "@/swagger";
import { defineStore } from "pinia";

type ServerInfoStoreState = {
  error: boolean;
  loading: boolean;
  showHidden: boolean;
  items: Feed[];
};

export const useFeedStore = defineStore({
  id: "server-info",
  state: () =>
    ({
      showHidden: false,
      error: false,
      loading: false,
      items: [],
    } as ServerInfoStoreState),
  actions: {
    async fetchServerInfo() {
      this.loading = true;
      this.items = await new FeedApi(conf())
        .showFeeds({ showHidden: true })
        .catch((e) => {
          console.log(e);
          this.error = true;
          return [];
        })
        .finally(() => {
          this.loading = false;
        });
    },
  },
});
