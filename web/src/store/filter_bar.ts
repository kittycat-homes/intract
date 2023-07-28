import { defineStore } from "pinia";

type FilterBarStoreState = {
  show: boolean;
};

export const useFilterBarStore = defineStore({
  id: "filter-bar",
  persist: true,
  state: () => {
    return { show: true } as FilterBarStoreState;
  },
});
