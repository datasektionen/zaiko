<template>
  <div>
    <table class="w-full border-collapse table-auto m-4 bg-(--zaiko-bg-0) text-(--zaiko-text) rounded-lg shadow-md p-4">
      <thead>
        <tr class="border-b-2 border-(--zaiko-border-color) text-(--zaiko-text-lc)">
          <td v-if="checkbox" class="p-2 border-b border-(--zaiko-bg-2) w-12">
            <input type="checkbox" :checked="allSelected" @change="toggleSelectAll" />
          </td>
          <td class="p-2 w-18">{{ Object.entries(columns)[0][1] }}</td>
          <td v-for="column in Object.entries(columns).slice(1)" :key="column[0]"
            class="p-2 text-left border-b border-(--zaiko-bg-2)">
            <p>{{ column[1] }}</p>
          </td>
        </tr>
      </thead>

      <tbody>
        <template v-for="(row, rowIndex) in processedRows" :key="rowIndex">
          <!-- Parent row -->
          <tr v-if="row.type === 'parent'" class="bg-(--zaiko-bg-0) font-semibold cursor-pointer"
            @click="toggleExpand(rowIndex)">
            <td v-if="checkbox" class="p-2 border-b border-(--zaiko-bg-2) w-8" @click.stop>
              <input type="checkbox" :checked="selectedRows.has(rowIndex)" @change.stop="toggleRow(rowIndex)" />
            </td>

            <td :colspan="Object.keys(columns).length + (settings ? 1 : 0)" class="p-2 border-b border-(--zaiko-bg-2)">
              <div class="flex items-center">
                <span class="mr-2">
                  <ChevronDownIcon class="w-6 h-6 inline-block" v-if="expanded.has(rowIndex)" />
                  <ChevronRightIcon class="w-6 h-6 inline-block" v-else />
                </span>
                <span>{{ row.data.name }}</span>
              </div>
            </td>
          </tr>

          <!-- Child rows -->
          <tr v-if="row.type === 'child' && expanded.has(row.parentIndex!)" v-for="(item, i) in row.data.items"
            :key="`${rowIndex}-${i}`">
            <td class="w-12"></td>
            <td v-if="checkbox" class="p-2 border-b border-(--zaiko-bg-0) w-8 pl-6">
              <input type="checkbox" :checked="selectedRows.has(`${rowIndex}-${i}`)"
                @change="toggleChildRow(rowIndex, i)" />
            </td>

            <slot :row="item" :columns="columns" name="row" />

            <td v-if="settings" class="m-0 p-2 w-8 border-b border-(--zaiko-bg-2) relative overflow-visible">
              <slot :row="item" :columns="columns" name="settings" />
            </td>
          </tr>
        </template>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts" generic="T extends Record<string, unknown>, S extends Record<string, unknown>">
import { ChevronDownIcon, ChevronRightIcon } from '@heroicons/vue/24/outline'
import { defineProps, defineEmits, ref, computed } from 'vue'

type TreeRow<T> = { name: string; items?: Array<T> }

const props = defineProps<{
  rows: Array<TreeRow<T>>,
  columns: S,
  checkbox?: boolean,
  settings?: boolean,
}>()

const emit = defineEmits<{
  (e: 'select', selected: Array<T | TreeRow<T>>): void
}>()

const selectedRows = ref<Set<string | number>>(new Set())
const expanded = ref<Set<number>>(new Set())

// Flatten tree rows into parent + child descriptors
const processedRows = computed(() =>
  props.rows.flatMap((row, index) => [
    { type: 'parent', data: row, index },
    row.items && row.items.length
      ? { type: 'child', data: row, parentIndex: index }
      : []
  ]).flat()
)

const allSelected = computed(() => selectedRows.value.size > 0 &&
  selectedRows.value.size ===
  props.rows.reduce(
    (acc, r) => acc + (r.items ? r.items.length : 0) + 1,
    0
  )
)

function toggleRow(index: number) {
  if (selectedRows.value.has(index)) selectedRows.value.delete(index)
  else selectedRows.value.add(index)
  emitSelected()
}

function toggleChildRow(parentIndex: number, childIndex: number) {
  const key = `${parentIndex}-${childIndex}`
  if (selectedRows.value.has(key)) selectedRows.value.delete(key)
  else selectedRows.value.add(key)
  emitSelected()
}

function toggleExpand(index: number) {
  if (expanded.value.has(index)) expanded.value.delete(index)
  else expanded.value.add(index)
}

function toggleSelectAll() {
  if (allSelected.value) selectedRows.value.clear()
  else {
    props.rows.forEach((r, i) => {
      selectedRows.value.add(i)
      r.items?.forEach((_, j) => selectedRows.value.add(`${i}-${j}`))
    })
  }
  emitSelected()
}

function emitSelected() {
  const selected: Array<T | TreeRow<T>> = []
  selectedRows.value.forEach(key => {
    if (typeof key === 'number') selected.push(props.rows[key])
    else {
      const [pi, ci] = key.split('-').map(Number)
      const parent = props.rows[pi]
      const child = parent.items?.[ci]
      if (child) selected.push(child)
    }
  })
  emit('select', selected)
}
</script>

<style scoped>
input[type="checkbox"] {
  accent-color: var(--zaiko-main-color);
  background-color: var(--zaiko-bg-2);
}
</style>
