<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const grid = ref([]);

async function get_random_word() {
    grid.value = await invoke("get_random_word");
}

async function speak_tts(text: string) {
    await invoke("speak_tts", { text: text});
}

onMounted(() => {
    get_random_word()
});

</script>
<template>
    <div class="grid-content">
        <div class="top-panel">
            <button @click="get_random_word">다음</button>
        </div>

        <div class="grid">
            <div
                v-for="(cell, index) in grid"
                :key="index"
                class="cell"
                @click="speak_tts(cell)"
            >
            {{ cell }}
            </div>
        </div>

    </div>
</template>

<style>
.grid-content {
    width: 100%;
    height: 100%;
    background-color: whitesmoke;
    display: flex;
    flex-direction: column;
}

.grid {
    padding: 10px;
    flex: 1;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: repeat(3, 1fr);
    gap: 5px;
}

.cell {
    display: flex;
    align-items: center;
    justify-content: center;
    background: white;
    font-size: 40px;
    font-weight: bold;
    border-radius: 5px;
    cursor: pointer;
}

.cell:hover {
    background: #ff9f00;  /* 배경색을 부드럽게 변경 */
    color: white;  /* 글자 색을 흰색으로 변경 */
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2); /* 부드러운 그림자 */
    transform: scale(1.01);  /* 살짝 커지는 효과 */   
}
</style>