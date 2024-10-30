import {createRouter, createWebHashHistory} from "vue-router";
import Bitcoin from "./components/Bitcoin.vue";
import TxBuilder from "./components/bitcoin/TxBuilder.vue";

export const routes = [
    {path: '/', component: TxBuilder},
    {path: '/tools', component: Bitcoin},
]

export const router = createRouter({
    history: createWebHashHistory(),
    routes,
})
