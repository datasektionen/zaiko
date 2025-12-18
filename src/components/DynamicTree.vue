<template>
  <div>
    <table
      class="w-full border-collapse table-auto m-4 bg-(--zaiko-bg-0) text-(--zaiko-text) rounded-lg shadow-md p-4"
    >
      <thead>
        <tr class="border-b-2 border-(--zaiko-bg-2) text-(--zaiko-text-lc)">
          <td class="p-2 w-18">{{ Object.entries(columns)[0][1] }}</td>
          <td
            v-for="column in Object.entries(columns).slice(1)"
            :key="column[0]"
            class="p-2 text-left border-b border-(--zaiko-bg-2)"
          >
            <p>{{ column[1] }}</p>
          </td>
          <td class="w-4"></td>
        </tr>
      </thead>
      <tbody>
        <template
          v-for="(row, rowIndex) in rows"
          :key="rowIndex"
          v-if="rows && node"
        >
          <tr
            class="p-2 border-b border-(--zaiko-bg-2)"
            v-if="(Object.values(row)[0] as string) !== ''"
          >
            <td
              class="p-1 w-14 cursor-pointer flex items-center"
              @click="toggleOpen(rowIndex)"
            >
              <ChevronDownIcon class="w-8 h-8" v-if="expanded.has(rowIndex)" />
              <ChevronRightIcon class="w-8 h-8" v-else />
            </td>
            <td>{{ row.name }}</td>
            <td></td>
            <td></td>
            <td>
              <slot :row="row" name="con" />
            </td>
          </tr>
          <tr
            v-for="(child, childIndex) in Object.values(row)[1]"
            :key="childIndex"
            v-if="expanded.has(rowIndex) || Object.values(row)[0] === ''"
            class="p-2 border-b border-(--zaiko-bg-2)"
          >
            <td class="w-12"></td>
            <slot :row="child as N" name="row" :container="row" />
          </tr>
        </template>
      </tbody>
    </table>
  </div>
</template>

<script
  setup
  lang="ts"
  generic="
    T extends Record<string, unknown>,
    S extends Record<string, unknown>,
    N extends Record<string, unknown>
  "
>
import { ChevronDownIcon, ChevronRightIcon } from '@heroicons/vue/24/outline'
import { defineProps, ref, watch } from 'vue'
import { useRoute } from 'vue-router'

const props = defineProps<{
  rows: Array<T>
  node: N
  columns: S
}>()

const route = useRoute()

watch(
  () => props.rows,
  newVal => {
    // Reset expanded state when rows change
    expanded.value = new Set()
    const containerName = (route.query.container as string) || ''
    newVal.findIndex((row, index) => {
      if (row.name === containerName) {
        expanded.value.add(index)
        return true
      }
      return false
    })
  },
)

const expanded = ref<Set<number>>(new Set())

const toggleOpen = (n: number) => {
  if (expanded.value.has(n)) {
    expanded.value.delete(n)
  } else {
    expanded.value.add(n)
  }
}
</script>

<style scoped>
input[type='checkbox'] {
  accent-color: var(--zaiko-main-color);
  background-color: var(--zaiko-bg-2);
}
</style>
