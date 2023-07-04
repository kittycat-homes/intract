import { createApp } from "vue";
import App from "./App.vue";
import "./registerServiceWorker";
import router from "./router";
import { createPinia } from "pinia";
import "@/assets/scss/styles.scss";
import { HiRss, HiHome } from "oh-vue-icons/icons";
import { OhVueIcon, addIcons } from "oh-vue-icons";

const pinia = createPinia();

addIcons(HiRss, HiHome);

createApp(App)
  .component("v-icon", OhVueIcon)
  .use(router)
  .use(pinia)
  .mount("#app");
