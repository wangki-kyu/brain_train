<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import MessageBox from './MessageBox.vue';

const messageBox = ref();
const idDev = ref(import.meta.env.MODE === 'development');

interface WordProblem {
  problem : string,
  problem_images : string[]
}

let word_problem = ref<WordProblem | null>(null);
const image_path = ref<string>("");
const router = useRouter();
const showSubMenu = ref<boolean>(false);

const navigateTo = (path : string) => {
  router.push(path);
}

const toggleSubMenu = () => {
  showSubMenu.value = !showSubMenu.value;
}

const category = ref("fruit");

async function GetImage(target: string) {
  word_problem.value = await invoke("get_words_from_file", { target : target });
}

async function GetImagePath(target: string) {
  image_path.value = await invoke("get_file_env", { target: target});
}

async function ImageClickEvnetHandler(image_path : string | undefined) {
  const answer = image_path?.split('.')[0];
  if (answer === word_problem.value?.problem) {
    showAlert('정답여부', '정답입니다.\r\n다음 문제로 넘어갑니다.');
    GetImage(category.value);
  } else {
    showAlert('정답여부', '틀렸습니다.\r\n다시 시도하세요');
  }
}

function showAlert(title: string, message: string) {
  console.log(messageBox.value);
  messageBox.value?.show_alert(title, message);
}

function change_category(text : string) {
  category.value = text;
  GetImagePath(category.value);
}

GetImagePath(category.value);

</script>

<template>
  <div class="container">
    <div class="top-panel">
      <button v-if="idDev" @click="showAlert('알림', '테스트')">Test</button>
      <button v-if="idDev" @click="change_category('family')">Content</button>
      <button @click="GetImage(category)">다음</button>
    </div>

    <div class="main-container">
      <!-- 사이드 패널 -->
      <div class="side-panel">
        <ul class="menu">
          <li class="menu-item" @click="toggleSubMenu()">낱말 맞추기</li>
          <transition name="fade">
            <ul v-if="showSubMenu" class="sub-menu">
              <li class="sub-menu-item" @click="navigateTo('/SubMenu1')">과일</li>
              <li class="sub-menu-item" @click="navigateTo('/SubMenu2')">가족</li>
            </ul>
          </transition>
          <li class="menu-item" @click="navigateTo('/Content')">Profile</li>
          <transition name="fade">
            <ul v-if="showSubMenu" class="sub-menu">
              <li class="sub-menu-item" @click="navigateTo('/SubMenu1')">과일</li>
              <li class="sub-menu-item" @click="navigateTo('/SubMenu2')">가족</li>
            </ul>
          </transition>
          <li class="menu-item" @click="navigateTo('/Settings')">Settings</li>
          <transition name="fade">
            <ul v-if="showSubMenu" class="sub-menu">
              <li class="sub-menu-item" @click="navigateTo('/SubMenu1')">과일</li>
              <li class="sub-menu-item" @click="navigateTo('/SubMenu2')">가족</li>
            </ul>
          </transition>
          <li class="menu-item" @click="navigateTo('/Help')">Help</li>
          <transition name="fade">
            <ul v-if="showSubMenu" class="sub-menu">
              <li class="sub-menu-item" @click="navigateTo('/SubMenu1')">과일</li>
              <li class="sub-menu-item" @click="navigateTo('/SubMenu2')">가족</li>
            </ul>
          </transition>
          
        </ul>
      </div>
      <div class="content-panel"> 
        <!-- main panel -->
        <div class="main-panel">
          <div class="image-container">
            <img :src="image_path + word_problem?.problem_images[0]" alt="image" @click="ImageClickEvnetHandler(word_problem?.problem_images[0])">
          </div>
          <div class="image-container">
            <img :src="image_path + word_problem?.problem_images[1]" alt="image" @click="ImageClickEvnetHandler(word_problem?.problem_images[1])">
          </div>
        </div>

        <!-- bottom panel -->
        <div class="bottom-panel">
          <p>{{ word_problem?.problem }}</p>
          <router-view></router-view>
        </div>

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

.side-panel {
  width: 20%;
  background-color: cornsilk;
  color: white;
  display: flex;
  justify-content: center;
  align-items: flex;
  border-radius: 15px;
}

.side-panel li {
  color: black;
}

.menu {
  list-style-type: none;
  padding: 0;
}

.menu-item {
  padding: 15px 0;
  cursor: pointer;
  transition: background-color 0.3s;
}

.menu-item:hover {
  background-color: antiquewhite;
}

.sub-menu {
  list-style-type: none;
  padding: 0;
  /* margin-left: 20px; */
}

.sub-menu-item {
  padding: 5px;
  cursor: pointer;
  transition: background-color 0.3s;
  display: flex;
  align-items: center;
}

.sub-menu-item:hover {
  background-color: grey;
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