import {createRouter, createWebHashHistory} from "vue-router";
import TxBuilder from "./components/bitcoin/TxBuilder.vue";
import Misc from "./components/Misc.vue";

export const routes = [
    {path: '/', component: TxBuilder},
    {path: '/misc', component: Misc},
]

export const router = createRouter({
    history: createWebHashHistory(),
    routes,
})
