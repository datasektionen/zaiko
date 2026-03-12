<template>
  <div
    class="flex justify-center align-center gap-4 mr-2 pl-1 pb-1 border-b-2 border-(--zaiko-main-color) max-w-full"
  >
    <component
      :is="icon"
      class="flex justify-center align-center max-w-8 max-h-8 w-8 h-8 text-(--zaiko-text)"
    />
    <RouterLink
      v-if="titleLink"
      :to="titleLink"
      class="m-0 mr-2 text-3xl font-semibold text-(--zaiko-text) font-left max-w-3xl text-ellipsis overflow-hidden"
    >
      {{ title }}
    </RouterLink>
    <h2
      class="m-0 mr-2 text-3xl font-semibold text-(--zaiko-text) font-left max-w-3xl text-ellipsis overflow-hidden"
      v-else
    >
      {{ title }}
    </h2>
  </div>
</template>

<script setup lang="ts">
import { defineProps, type FunctionalComponent } from 'vue'

defineProps<{
  title: string
  icon: FunctionalComponent
  titleLink?: string
}>()
</script>

<style scoped></style>
