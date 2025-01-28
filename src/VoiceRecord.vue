<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useRoute } from 'vue-router';

const is_record = ref(false);
const route = useRoute();
const category = ref(route.params.category[0]);
const word_list = ref<string[] | null>(null);
const show_word = ref("");
const word_index = ref(0);
const is_playing = ref(false);

function toggle_record() {
    // 녹음 하기 
    if (!is_record.value) {
        start_record();
    } else {
        pause_record();
    }

    is_record.value = !is_record.value;
}

watch(
  () => route.params.category,
  async (newCateogry) => {
    category.value = newCateogry[0];
    await get_json_file_data(category.value);

    word_index.value = 0;

    if (word_list.value && word_list.value.length > 0) {
        show_word.value = word_list.value[word_index.value];
        word_index.value++;
    }
  }
);

async function get_json_file_data(category : string) {
    word_list.value = await invoke("get_record_json_data", { category : category});
    console.log(word_list.value);
}

function show_next_word() {
    if (word_list.value && word_list.value.length > 0) {
        if (word_index.value == word_list.value.length - 1)
            word_index.value = 0;

        show_word.value = word_list.value[word_index.value];
        console.log(show_word);
        word_index.value += 1;
    } else {
        show_word.value = "";
    }   
}

async function start_record() {
    await invoke("start_record");
    console.log("record start");
}

async function pause_record() {
    await invoke("pause_record");
    console.log("record pause");
}

async function test_func()  {
    await invoke("test_func");
    console.log("test func called");
}

async function stream_record() {
    is_playing.value = true;
    invoke("stream_record").then(() => {
        console.log("called stream_record");
        is_playing.value = false;
    }).catch((err) => {
        console.error("Error durint stream_record:", err);
        is_playing.value = false;
    });
}

async function speak_tts(text: string) {
    await invoke("speak_tts", { text: text});
}

function handleSpacebar(event: KeyboardEvent) {
    if (event.code === 'Space') {
        console.log('Spacebar pressed');
        toggle_record();
    }
}

onMounted(async () => {
    await get_json_file_data(category.value);
    console.log(word_list.value);
    if (word_list.value && word_list.value.length > 0) {
        show_word.value = word_list.value[word_index.value];
        word_index.value++;
    }

    window.addEventListener('keydown', handleSpacebar);
});

onBeforeUnmount(() => {
    // 키보드 이벤트 리스너 제거
    window.removeEventListener('keydown', handleSpacebar);
});

</script>

<template>
    <div class="conatiner">
        <div class="top-panel">
            <button @click="show_next_word">다음</button>
        </div>
        <div class="top-content">
            <p @click="speak_tts(show_word)">{{ show_word }}</p>
        </div>

        <div class="bottom-content">
            <button class="record-button" @click="toggle_record">
                <p v-if="is_record">정지</p>
                <p v-else>녹음</p>
            </button>
            <button class="play-button" @click="stream_record">
                <p v-if="is_playing" style="font-size: 20px;">재생 중..</p>
                <p v-else>재생</p>
            </button>

        </div>
    </div>
</template>

<style scoped>
.conatiner {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.top-panel {
    background-color: whitesmoke;
}

.top-content {
    background-color: whitesmoke;
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
}

.top-content p {
    font-size: 70px;
    cursor: pointer;
}

.bottom-content {
    background-color: whitesmoke;
    height: 40%;
    display: flex;
    justify-content: space-evenly;
    align-items: center;
}

.record-button {
    height: 100px;
    width: 100px;
    background-color: #ff4d4d;
    color: white;
    border: none;
    border-radius: 100px;
    cursor: pointer;
    overflow: hidden;
    box-shadow: 0 4px 10px rgba(255, 77, 77, 0.5);
    transition: transform 0.2s, background-color 0.2s;
    display: flex;
    justify-content: center;
    align-items: center;
}

.record-button:hover {
    background-color: #e63946;
    transform: scale(1.1);
}

.record-button p {
    font-size: 30px;
}

/* 버튼 스타일 */
.play-button {
  width: 100px;
  height: 100px;
  background: #fff;
  border: none;
  border-radius: 50%;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
  transition: transform 0.2s, background-color 0.2s;
}

.play-button:hover {
  background: #f0f0f0;
  transform: scale(1.1);
}

.play-button:active {
  transform: scale(0.95);
}

.play-button p {
    font-size: 30px;
}

/* 삼각형 모양 */
.triangle {
  width: 0;
  height: 0;
  border-style: solid;
  border-width: 15px 0 15px 30px; /* 위, 오른쪽, 아래, 왼쪽 */
  border-color: transparent transparent transparent #282c34;
  transition: transform 0.2s;
}

</style>