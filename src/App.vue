<script lang="ts" setup>
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import {
  TEXT_CHANGED,
  IMAGE_CHANGED,
  listenText,
  listenImage,
  writeText, writeImage,
} from "tauri-plugin-clipboard-api";
import {onMounted, onUnmounted, ref} from "vue";
import Card from "./components/Card.vue";
import IconClose from "./components/icons/IconClose.vue";

let optStatus = ref(0)
let pasteItems = ref([])
let tauriTextUnlisten: UnlistenFn;
let tauriImageUnlisten: UnlistenFn;
let textUnlisten: () => void;
let imageUnlisten: () => void;

const stopListening = () => {
  imageUnlisten();
  textUnlisten();
  tauriTextUnlisten();
  tauriImageUnlisten();
}
const startListening = async () => {
  tauriTextUnlisten = await listen(TEXT_CHANGED, (event) => {
    pasteItems.value.push({
      type: 1,
      value: (event.payload as any).value,
      date: new Date()
    })
  });
  tauriImageUnlisten = await listen(IMAGE_CHANGED, (event) => {
    pasteItems.value.push({
      type: 2,
      value: 'data:image/png;base64,' + (event.payload as any).value,
      date: new Date()
    })
  });
  imageUnlisten = listenImage();
  textUnlisten = listenText();
}

onMounted(() => {
  startListening();
})
onUnmounted(() => {
  stopListening();
})

const clickCopy = async (item) => {
  if (item.type === 1) {
    await writeText(item.value)
  } else {
    await writeImage(item.value.replace('data:image/png;base64,', ''))
  }
  optStatus.value = 1
  setTimeout(() => {
    optStatus.value = 0
  }, 3000)
}

const clickCloseAlert = () => {
  optStatus.value = 0
}
</script>
<template>
  <div data-tauri-drag-region style="height:30px"></div>
  <div v-if="optStatus > 0" class="bg-indigo-900 text-center my-2 lg:px-4">
    <div class="p-2 bg-indigo-800 items-center text-indigo-100 leading-none lg:rounded-full flex lg:inline-flex" role="alert">
      <span class="flex rounded-full bg-indigo-500 uppercase px-2 py-1 text-xs font-bold mr-3">Notice</span>
      <span class="font-semibold mr-2 text-left flex-auto">Success</span>
      <button @click="clickCloseAlert">
        <icon-close></icon-close>
      </button>
    </div>
  </div>
  <div class="container max-auto px-4 pb-16 flex flex-col-reverse">
    <div v-if="pasteItems.length > 0" v-for="item in pasteItems" class="card">
      <card :item="item" @click-copy="clickCopy(item)"></card>
      <div class="text-center text-xs text-white my-1">{{item.date}}</div>
    </div>
    <div v-else class="text-center text-xs text-white my-1">
      <p>Clipboard empty.</p>
    </div>
  </div>
</template>

<style>
body {
  @apply bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500;
}
</style>
