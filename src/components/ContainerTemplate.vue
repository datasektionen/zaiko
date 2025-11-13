<template>
  <div class="flex flex-col w-fit h-fit bg-(--zaiko-bg-1) rounded-2xl p-2 m-4">
    <div class="flex justify-between align-center">
      <div class="flex justify-start align-center">
        <TitleSmall :title="title" :icon="icon" />
        <button
          v-if="buttonLeftIcon"
          @click="emit('buttonLeft')"
          class="flex justify-center aspect-square align-center p-1.5 border-none cursor-pointer bg-(--zaiko-main-color) rounded-lg text-(--zaiko-bg-1) ml-4"
        >
          <component :is="buttonLeftIcon" class="w-8 h-8" />
        </button>
      </div>
      <div class="flex justify-start align-center pr-4">
        <button
          v-if="buttonRightIcon"
          @click="emit('buttonRight')"
          class="flex justify-center aspect-square align-center p-1.5 border-none cursor-pointer bg-(--zaiko-main-color) rounded-lg text-(--zaiko-bg-1) ml-4"
        >
          <component :is="buttonRightIcon" class="w-8 h-8" />
        </button>
      </div>
    </div>
    <div class="flex flex-col gap-4 pt-4 m-1 md:pr-2 w-fit">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import TitleSmall from '@/components/TitleSmall.vue'
import { defineProps, defineEmits, type FunctionalComponent } from 'vue'
defineProps<{
  title: string
  buttonLeftIcon?: FunctionalComponent
  buttonRightIcon?: FunctionalComponent
  icon: FunctionalComponent
}>()
const emit = defineEmits(['buttonLeft', 'buttonRight'])
</script>

<style scoped></style>
