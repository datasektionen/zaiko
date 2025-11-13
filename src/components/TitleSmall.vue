<template>
  <div class="flex justify-center align-center gap-2 mr-2 pl-1 pb-1 w-fit">
    <component
      :is="icon"
      class="flex justify-center align-center max-w-8 max-h-8 text-(--zaiko-text)"
    />
    <h4
      class="m-0 text-xl font-bold text-(--zaiko-text) font-left w-fit text-nowrap"
    >
      {{ title }}
    </h4>
  </div>
</template>

<script setup lang="ts">
import { defineProps, type FunctionalComponent } from 'vue'

defineProps<{
  title: string
  icon: FunctionalComponent
}>()
</script>

<style scoped></style>
