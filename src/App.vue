<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import MessageBox from './MessageBox.vue';

const messageBox = ref();
// const idDev = ref(import.meta.env.MODE === 'development');

// interface WordProblem {
//   problem : string,
//   problem_images : string[]
// }

// let word_problem = ref<WordProblem | null>(null);
const image_path = ref<string>("");
const router = useRouter();
const word_quiz_menu_toggle = ref(false);
const void_record_menu_toggle = ref(false);
const word_view_toggle = ref(false);

function toggle_word_quiz_menu() {
  word_quiz_menu_toggle.value = !word_quiz_menu_toggle.value;
}

function toggle_voice_record_menu() {
  void_record_menu_toggle.value = !void_record_menu_toggle.value;
}

function word_view_menu() {
  word_view_toggle.value = !word_view_toggle.value;
}

// const navigateTo = (path : string) => {
//   router.push(path);
// }

const category = ref("fruit");

// async function GetImage(target: string) {
//   word_problem.value = await invoke("get_words_from_file", { target : target });
// }

async function GetImagePath(target: string) {
  image_path.value = await invoke("get_file_env", { target: target});
}

// async function ImageClickEvnetHandler(image_path : string | undefined) {
//   const answer = image_path?.split('.')[0];
//   if (answer === word_problem.value?.problem) {
//     showAlert('정답여부', '정답입니다.\r\n다음 문제로 넘어갑니다.');
//     GetImage(category.value);
//   } else {
//     showAlert('정답여부', '틀렸습니다.\r\n다시 시도하세요');
//   }
// }

// function showAlert(title: string, message: string) {
//   console.log(messageBox.value);
//   messageBox.value?.show_alert(title, message);
// }

function change_category(path : string,text : string) {
  console.log(`change_category clicked, {text}`);
  console.log(text);
  category.value = text;
  router.push({ path: `/${path}/${text}`});
}

function go_router(path: string) {
  router.push({path: path});
}

GetImagePath(category.value);

</script>

<template>
  <div class="container">
    <!-- <div class="top-panel">
      <button v-if="idDev" @click="showAlert('알림', '테스트')">Test</button>
      <button v-if="idDev" @click="change_category('family')">Content</button>
      <button @click="GetImage(category)">다음</button>
    </div> -->

    <div class="main-container">
      <div class="side-menu">
        <div class="side-menu-element" @click="toggle_word_quiz_menu">
          <p>낱말 퀴즈</p>
        </div>
        <div v-if="word_quiz_menu_toggle" class="sub-menu">
          <p @click="change_category('WordQuiz','fruit')">과일</p>
          <p @click="change_category('WordQuiz', 'family')">가족</p>
        </div>
        <div class="side-menu-element" @click="toggle_voice_record_menu">
          <p>단어 말하기 연습</p>
        </div>
        <div v-if="void_record_menu_toggle" class="sub-menu">
          <p @click="change_category('VoiceRecord', 'fruit')">과일</p>
          <p @click="change_category('VoiceRecord', 'family')">가족</p>
          <!-- <p @click="change_category('VoiceRecord', '한글')">한글</p> -->
        </div>
        <div class="side-menu-element" @click="word_view_menu">
          <p>단어 전체 보기</p>
        </div>
        <div v-if="word_view_toggle" class="sub-menu">
          <p @click="go_router('/WordView')">단어 전체 보기</p>
          <!-- <p @click="change_category('VoiceRecord', '한글')">한글</p> -->
        </div>
      </div>

      <div class="content-panel"> 
        <router-view></router-view>
        <!-- main panel -->
        <!-- <div class="main-panel">
          <div class="image-container">
            <img :src="image_path + word_problem?.problem_images[0]" alt="image" @click="ImageClickEvnetHandler(word_problem?.problem_images[0])">
          </div>
          <div class="image-container">
            <img :src="image_path + word_problem?.problem_images[1]" alt="image" @click="ImageClickEvnetHandler(word_problem?.problem_images[1])">
          </div>
        </div> -->

        <!-- bottom panel -->
        <!-- <div class="bottom-panel">
          <p>{{ word_problem?.problem }}</p>
        </div> -->

      </div>
    </div>

    <MessageBox ref="messageBox"/>
  </div>
  
</template>


<style>

html, body {
  margin: 0;
  padding: 0;
  height: 100%;
  width: 100%;
}

.container {
  height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column;
}

/* test */
.test {
  flex: 1;
  background-color: #4CAF50;
}

.main-container {
  display: flex;
  flex: 1;
}

.side-menu {
  display: flex;
  flex-direction: column;
  background-color: #dcdcdc;
  width: 150px;
  padding: 20px;
}

.side-menu-element {
  width: 100%;
  height: 50px;
  background-color: #dcdcdc;
  display: flex;
  justify-content: center;
  align-items: center;
}

.side-menu-element:hover {
  background-color: aquamarine;
  cursor: pointer;
}

.side-menu-element p {
  font-size: 20px;
  color: gray;
}

.sub-menu {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.sub-menu p {
  font-size: 15px;
  width: 100px;
  height: 50px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.sub-menu p:hover {
  background-color: aqua;
  cursor: pointer;
}

.top-panel {
  background-color: antiquewhite;
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
  background-color: aqua;
  flex: 1;
  display: flex;
}

.content-panel {
  display: flex;
  flex-direction: column;
  flex: 1;
  background-color: whitesmoke;
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
  object-fit: cover;
  cursor: pointer;
}

.bottom-panel {
  background-color: white;
  display: flex;
  justify-content: center;
  align-items: center;
  height: 120px;
}

.bottom-panel p {
  font-size: 70px;
}


</style>