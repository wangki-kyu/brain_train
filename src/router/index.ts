import { createWebHistory, createRouter, RouteRecordRaw } from "vue-router";
import Test from "@/Test.vue";
import Content from "@/Content.vue";

const routes: RouteRecordRaw[] = [
    {
        path : "/Test",
        name: "Test",
        component: Test,
    },
    {
        path : "/Content",
        name: "Content",
        component: Content,
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;