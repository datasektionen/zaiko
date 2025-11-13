<template>
  <div
    :class="
      color +
      ' flex flex-col rounded-xl items-center justify-center md:aspect-square gap-4 shadow-md min-w-38'
    "
  >
    <h5 class="text-4xl font-semibold m-0 text-(--zaiko-bg-1)">
      {{ amount || 'N/A' }}
    </h5>
    <h6 class="text-sm m-0 text-(--zaiko-bg-1)">{{ title }}</h6>
  </div>
</template>

<script setup lang="ts">
import { computed, defineProps } from 'vue'

const props = defineProps<{
  title: string
  amount?: number | string | null
  good?: boolean
}>()

const color = computed<string>(() => {
  return props.good ? 'bg-(--zaiko-main-color)' : 'bg-(--zaiko-bad-color)'
})
</script>

<style scoped></style>
