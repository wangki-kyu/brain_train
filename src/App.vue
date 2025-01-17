<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface WordProblem {
  problem : string,
  problem_images : string[]
}

let word_problem = ref<WordProblem | null>(null);
const image_path = ref<string>("");

async function GetImage() {
  word_problem.value = await invoke("get_words_from_file");
}

async function GetImagePath() {
  image_path.value = await invoke("get_file_env");
}

async function ImageClickEvnetHandler(image_path : string | undefined) {
  const answer = image_path?.split('.')[0];
  if (answer === word_problem.value?.problem) {
    alert('정답입니다');
    GetImage();
  } else {
    alert('틀렸습니다.');
  }
}

GetImagePath();

</script>

<template>
  <div class="container">
    <div class="top-panel">
      <button @click="GetImage">다음</button>

    </div>

    <div class="main-container">
      <!-- 사이드 패널 -->
      <div class="side-panel">
        
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
        </div>

      </div>
    </div>
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
  background-color: black;
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
  object-fit: contain;
  cursor: pointer;
}

.bottom-panel {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 120px;
}

.bottom-panel p {
  font-size: 70px;
}


</style>