import { createRouter, createWebHashHistory } from "vue-router";

export default createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", component: () => import("../views/Transactions.vue") },
    { path: "/stats", component: () => import("../views/Stats.vue") },
    { path: "/settings", component: () => import("../views/Settings.vue") },
    { path: "/:pathMatch(.*)*", redirect: "/" },
  ],
});
