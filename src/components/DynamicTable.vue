<template>
  <div>
    <table
      class="w-full border-collapse table-auto m-4 bg-(--zaiko-bg-0) text-(--zaiko-text) rounded-lg shadow-md p-4">
      <thead>
        <tr class="border-b-2 border-(--zaiko-border-color) text-(--zaiko-text-lc)">
          <td v-if="checkbox" class="p-2 border-b border-(--zaiko-bg-2) w-8">
            <input
              type="checkbox"
              :checked="allSelected"
              @change="toggleSelectAll"
            />
          </td>
          <td v-for="column in Object.entries(columns)" :key="column[0]"
            class="p-2 text-left border-b border-(--zaiko-bg-2)">
            <p>{{ column[1] }}</p>
          </td>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(row, rowIndex) in rows" :key="rowIndex">
          <td v-if="checkbox" class="p-2 border-b border-(--zaiko-bg-2) w-8">
            <input
              type="checkbox"
              class="border-0"
              :checked="selectedRows.has(rowIndex)"
              @change="toggleRow(rowIndex)"
            />
          </td>
          <slot :row="row" :columns="columns" name="row" />
          <td v-if="settings" class="m-0 p-2 w-8 border-b border-(--zaiko-bg-2) relative overflow-visible">
            <slot :row="row" :columns="columns" name="settings" />
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts" generic="T extends Record<string, unknown>, S extends Record<string, unknown>">
import { defineProps, defineEmits, ref, computed } from 'vue'
const props = defineProps<{
  rows: Array<T>,
  columns: S,
  checkbox?: boolean,
  settings?: boolean,
}>()

const emit = defineEmits<{
  (e: 'select', selected: Array<T>): void
}>()

const selectedRows = ref<Set<number>>(new Set())

const allSelected = computed(() =>
  props.rows.length > 0 && selectedRows.value.size === props.rows.length
)

function toggleRow(index: number) {
  if (selectedRows.value.has(index)) {
    selectedRows.value.delete(index)
  } else {
    selectedRows.value.add(index)
  }
  emitSelected()
}

function toggleSelectAll() {
  if (allSelected.value) {
    selectedRows.value.clear()
  } else {
    props.rows.forEach((_, i) => selectedRows.value.add(i))
  }
  emitSelected()
}

function emitSelected() {
  const selected = Array.from(selectedRows.value).map(i => props.rows[i])
  emit('select', selected)
}
</script>

<style scoped>
input[type="checkbox"] {
  accent-color: var(--zaiko-main-color);
  background-color: var(--zaiko-bg-2);
}
</style>
