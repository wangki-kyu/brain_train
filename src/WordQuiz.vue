<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import MessageBox from './MessageBox.vue';
import { useRoute } from "vue-router";


const idDev = ref(import.meta.env.MODE === 'development');
const messageBox = ref();

interface WordProblem {
  problem : string,
  problem_images : string[],
  answer_list : string[]
}

let word_problem = ref<WordProblem | null>(null);
const image_path = ref<string>("");
const route = useRoute();
const category = ref(route.params.category[0]);

async function GetImage(target: string) {
  console.log(`{impage_path}`);
  console.log(category.value);
  word_problem.value = await invoke("get_words_from_file", { target : target });
}

async function GetImagePath(target: string) {
  console.log(target);
  image_path.value = await invoke("get_file_env", { target: target});
}

async function ImageClickEvnetHandler(image_path : string | undefined) {
  // const answer = image_path?.split('.')[0];
  const answer = image_path;
  if (answer === word_problem.value?.problem) {
    // showAlert('정답여부', '정답입니다.\r\n다음 문제로 넘어갑니다.');
    GetImage(category.value);
  } else {
    // showAlert('정답여부', '틀렸습니다.\r\n다시 시도하세요');
  }
}

watch(
  () => route.params.category,
  (newCateogry) => {
    category.value = newCateogry[0]
    GetImagePath(category.value);
    GetImage(category.value);
  }
);

async function loadImageAsBase64(imagePath: string): Promise<string> {
  const response = await fetch(imagePath);
  const blob = await response.blob();
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onloadend = () => resolve(reader.result as string);
    reader.onerror = reject;
    reader.readAsDataURL(blob);
  });
}

function showAlert(title: string, message: string) {
  console.log(messageBox.value);
  messageBox.value?.show_alert(title, message);
}

async function speak_tts(text: string | undefined) {
    await invoke("speak_tts", { text: text});
}

//function change_category(text : string) {
//  category.value = text;
// 
 // GetImagePath(category.value);
//}

const test_image_path = ref("");

async function test_func() {
  try {
    // test_image_path.value = await invoke("get_image_url");
    test_image_path.value = await loadImageAsBase64("D:\\work\\rust\\brain-train\\src-tauri\\target\\release\\images\\family\\김우준.jpg");
    console.log(test_image_path.value);  
  } catch (error) {
    
  }
}

onMounted(() => {
  GetImagePath(category.value);
  GetImage(category.value);
});

</script>

<template>
    <div class="container">
        <div class="top-panel">
            <button v-if="idDev" @click="showAlert('알림', '테스트')">Test</button>
            <button v-if="idDev" @click="test_func">Content</button>
            <button @click="GetImage(category)">다음</button>
        </div>

        <div class="main-panel">
            <div class="image-container">
                <img :src="word_problem?.problem_images[0]" alt="image"
                    @click="ImageClickEvnetHandler(word_problem?.answer_list[0])">
            </div>
            <div class="image-container">
                <img :src="word_problem?.problem_images[1]" alt="image"
                    @click="ImageClickEvnetHandler(word_problem?.answer_list[1])">
            </div>
            <!-- <div class="image-container">
                <img src="D:\work\rust\brain-train\src-tauri\target\release\images\family\김우준.jpg" alt="image"
                    @click="ImageClickEvnetHandler(word_problem?.problem_images[1])">
            </div>
            <div class="image-container">
                <img :src="test_image_path" alt="image"
                    @click="ImageClickEvnetHandler(word_problem?.problem_images[1])">
            </div> -->
        </div>

        <!-- bottom panel -->
        <div class="bottom-panel">
            <p @click="speak_tts(word_problem?.problem)">{{ word_problem?.problem }}</p>
        </div>

        <MessageBox ref="messageBox"/>
    </div>

</template>

<style>
.container {
  display: flex;
  flex-direction: column;
  width: 100%;
}

.top-panel {
  background-color: whitesmoke;
  height: 50px;
  display: flex;
  justify-content: right;
  align-items: center;
  padding: 10px;
}

.top-panel button {
  height: 40px;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  font-size: 16px;
  transition: background-color 0.3s ease;
  background-color: #4CAF50; /* Green */
  padding: 10px 20px;
}

.main-panel {
  width: 100%;
  background-color: whitesmoke;
  flex: 1;
  display: flex;
}

.content-panel {
  display: flex;
  flex-direction: column;
  flex: 1;
  background-color: #4CAF50;
}

.image-container {
  justify-content: center;
  align-items: center;
  background-color: azure;
  flex: 1;
  margin: 20px;
  border-radius: 20px;  
  border: 2px solid white;
}

.image-container:hover {
  border-color: red;
  border-width: 4px;
  transform: scale(1.05);
}

.image-container img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  cursor: pointer;
  aspect-ratio: 16/9;
}

.bottom-panel {
  background-color: whitesmoke;
  display: flex;
  justify-content: center;
  align-items: center;
  height: 120px;
}

.bottom-panel p {
  font-size: 70px;
  cursor: pointer;
}
</style>