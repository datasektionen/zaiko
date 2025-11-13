<template>
  <div>
    <RouterLink
      v-if="link"
      :to="link"
      :class="
        'flex justify-center gap-2 p-4 pl-6 pr-8 text-lg text-(--zaiko-text-hc) rounded-xl hover:underline cursor-pointer' +
        (bad ? ' bg-(--zaiko-bad-color)' : '  bg-(--zaiko-main-color)')
      "
    >
      <component :is="icon" class="w-6 h-6" />
      <p>{{ title }}</p>
    </RouterLink>
    <button
      @click="emit('click')"
      v-else
      :class="
        'flex justify-center gap-2 p-4 pl-6 pr-8 text-lg text-(--zaiko-text-hc) rounded-xl cursor-pointer' +
        (bad ? ' bg-(--zaiko-bad-color)' : '  bg-(--zaiko-main-color)')
      "
    >
      <component :is="icon" class="w-6 h-6" />
      <p>{{ title }}</p>
    </button>
  </div>
</template>

<script setup lang="ts">
import type { FunctionalComponent } from 'vue'
import { RouterLink } from 'vue-router'

const props = defineProps<{
  title: string
  icon: FunctionalComponent
  link?: string
  bad?: boolean
}>()

const emit = defineEmits<{
  (e: 'click'): void
}>()
</script>

<style scoped></style>
