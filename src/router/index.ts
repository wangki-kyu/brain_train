import { createWebHistory, createRouter, RouteRecordRaw } from "vue-router";
import WordQuiz from "@/WordQuiz.vue";
import VoiceRecord from "@/VoiceRecord.vue";
import WordView from "@/WordView.vue";

const routes: RouteRecordRaw[] = [
    {
        path : "/WordQuiz/:category+",
        name: "WordQuiz",
        component: WordQuiz,
    },
    {
        path : "/VoiceRecord/:category+",
        name: "VoiceRecord",
        component: VoiceRecord,
    },
    {
        path : "/WordView",
        name: "WordView",
        component: WordView,
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;