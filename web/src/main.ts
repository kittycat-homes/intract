import { createApp } from "vue";
import App from "./App.vue";
import "./registerServiceWorker";
import router from "./router";
import { createPinia } from "pinia";
import "@/assets/scss/styles.scss";

const pinia = createPinia();

createApp(App).use(router).use(pinia).mount("#app");
