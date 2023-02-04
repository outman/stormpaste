<template>
  <div class="p-2 mx-auto bg-white rounded-lg shadow-lg flex items-center">
    <div class="relative w-full text-slate-500 text-base">
        <textarea v-if="item.type === 1" class="w-full p-1" rows="4" readonly="true">{{item.value}}</textarea>
        <img v-else :src="item.value" class="paste-image">
        <button @click="emit('clickCopy')" class="absolute right-1 rounded p-1 top-1 bg-fuchsia-500 text-white">
          <icon-copy></icon-copy>
        </button>
    </div>
  </div>
</template>

<script setup>
import IconCopy from "./icons/IconCopy.vue";

defineProps({
  item: Object
})
const emit = defineEmits(["clickCopy"])
</script>

<style scoped>
textarea {
  outline: 0px !important;
  outline-offset: 0px !important;
  box-shadow: none !important;
  resize: none;
  border: none !important;
}

.paste-image {
  max-height: 100px;
}
</style>