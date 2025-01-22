<script setup lang="ts">
import { ref } from 'vue';

const title = ref("");
const message = ref("");
const dialogRef = ref<HTMLDialogElement | null>(null);

function show_alert(_title : string, _message: string) {
    title.value = _title;
    message.value = _message;
    dialogRef.value?.showModal();
}

function closeDialog() {
    dialogRef.value?.close();
}

defineExpose({
    show_alert
})
</script>

<template>
    <dialog ref="dialogRef" class="alert-dialog">
        <div class="alert-container">
            <h3>{{ title }}</h3>
            <p>{{ message }}</p>
            <div class="button-container">
                <button @click="closeDialog">닫기</button>
            </div>
        </div>
        
    </dialog>
</template>

<style>
.alert-dialog {
    width: 300px;
    height: 200px;
    padding: 0;
    border: none;
    border-radius: 10px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.alert-container {
  width: 100%;
  height: 100%;
  background-color: whitesmoke;
  padding: 20px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: start;
}

.alert-container h3 {
  margin: 0;
  margin-bottom: 10px;
}

.alert-container p {
  margin: 0;
  margin-bottom: 20px;
}

.alert-container button:hover {
  background-color: #45a049;
}

.button-container {
    width: 100%;
    display: flex;
    justify-content: flex-end;
    margin-top: 40px;
}

.button-container button {
    padding: 10px 20px;
    border: none;
    border-radius: 10px;
    background-color: grey;
    color: white;
    cursor: pointer;
    transition: background-color 0.3s ease;
}

.button-container button:hover {
  background-color: #45a049;
}

</style>