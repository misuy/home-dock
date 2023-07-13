import { createApp } from "vue"
import { createRouter, createWebHistory } from "vue-router"
import StorageArea from "./components/StorageArea.vue"
import App from "./App.vue"


const routes = [
    {path: "/storage/:storage_path*", component: StorageArea},
]

const router = createRouter({
    history: createWebHistory(),
    routes: routes,
})


const app = createApp(App);
app.use(router);
app.mount("#app");
