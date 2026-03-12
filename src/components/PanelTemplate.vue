<template>
  <div
    class="flex flex-col h-fit bg-(--zaiko-bg-2) rounded-2xl shadow-md md:p-3 p-2 md:m-6 m-4"
  >
    <div class="flex justify-between align-center flex-col md:flex-row">
      <div class="flex justify-start align-center flex-col md:flex-row">
        <TitleBig :title="title" :icon="icon" :titleLink="titleLink" />
        <button
          v-if="buttonLeftIcon && !buttonLeftRestricted"
          @click="emit('buttonLeft')"
          class="flex justify-center self-end md:self-baseline aspect-square align-center p-1.5 border-none cursor-pointer bg-(--zaiko-main-color) rounded-lg text-(--zaiko-bg-1) mr-4 md:mr-0 md:ml-4 w-12"
        >
          <component :is="buttonLeftIcon" class="w-8 h-8" />
        </button>
      </div>
      <div class="flex justify-end md:justify-start align-center pr-4">
        <button
          v-if="buttonRightIcon && !buttonRightRestricted"
          @click="emit('buttonRight')"
          class="flex justify-center aspect-square align-center p-1.5 border-none cursor-pointer bg-(--zaiko-main-color) rounded-lg text-(--zaiko-bg-1) ml-4 w-12 h-12"
        >
          <component :is="buttonRightIcon" class="w-8 h-8" />
        </button>
      </div>
    </div>
    <div class="flex flex-col gap-4 pt-4 m-4 md:pr-2">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import TitleBig from '@/components/TitleBig.vue'
import { defineProps, defineEmits, type FunctionalComponent } from 'vue'
defineProps<{
  title: string
  buttonLeftIcon?: FunctionalComponent
  buttonLeftRestricted?: boolean
  buttonRightIcon?: FunctionalComponent
  buttonRightRestricted?: boolean
  icon: FunctionalComponent
  titleLink?: string
}>()
const emit = defineEmits(['buttonLeft', 'buttonRight'])
</script>

<style scoped></style>
