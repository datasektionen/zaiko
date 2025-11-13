<template>
  <div>
    <div class="relative inline-block">
      <EllipsisVerticalIcon
        class="h-6 w-6 text-(--zaiko-text) cursor-pointer"
        @click="isOpen = !isOpen"
      />
      <div class="fixed z-10 bg-(--zaiko-bg-1)">
        <div
          v-if="isOpen"
          v-on-click-outside="() => (isOpen = !isOpen)"
          class="absolute top-0 right-5 bg-(--zaiko-bg-1) border border-(--zaiko-bg-2) rounded-md shadow-md w-fit min-w-36 z-10"
        >
          <ul class="flex flex-col gap-2">
            <li
              class="p-2 hover:bg-(--zaiko-bg-2) hover:cursor-pointer"
              v-for="row in Object.keys(rows)"
              @click="click(row)"
              :key="row"
            >
              {{ rows[row] }}
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts" generic="T extends Record<string, string>">
import { EllipsisVerticalIcon } from '@heroicons/vue/24/outline'
import { vOnClickOutside } from '@vueuse/components'
import { ref } from 'vue'

const isOpen = ref(false)

const click = (row?: string) => {
  if (row) emit('select', row)
  isOpen.value = false
}

const props = defineProps<{
  rows: T
}>()

// event emits on selected li and returns row name
const emit = defineEmits<{
  (e: 'select', selected: string): void
}>()
</script>

<style scoped></style>
