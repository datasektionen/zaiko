<template>
  <div class="panelContainer">
    <div class="panelHeader">
      <TitleBig :title="title">
        <slot name="icon" />
      </TitleBig>
      <button v-if="button" @click="emit('button')">
        <PlusIcon />
      </button>
    </div>
    <div class="panelContent">
      <slot name="content" />
    </div>
  </div>
</template>

<script setup lang="ts">
import TitleBig from '@/components/TitleBig.vue'
import { PlusIcon } from '@heroicons/vue/24/outline'
import { defineProps, defineEmits } from 'vue'
defineProps<{ title: string, button?: boolean }>()
const emit = defineEmits(["button"])
</script>

<style scoped>
button {
  all: unset;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 0.4rem;
  margin: 0;
  background-color: transparent;
  border: none;
  cursor: pointer;
  background-color: #80CBC3;
  border-radius: 8px;
  color: #FAFAFA;
}

svg {
  width: 32px;
  height: 32px;
}

.panelHeader {
  display: flex;
  justify-content: start;
  align-items: center;
  gap: 1rem;
}

@media (max-width: 768px) {
  .panelContainer {
    max-width: 100vw;
  }
}
</style>
