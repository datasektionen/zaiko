<template>
  <div>
    <div class="submit justify-end pr-6 pt-4 sticky top-4 right-6">
      <button
        @click="TakeStock()"
        class="flex items-center gap-2 p-2 bg-(--zaiko-main-color) text-(--zaiko-text-hc) rounded-md hover:cursor-pointer"
      >
        <DocumentCheckIcon class="w-8 aspect-square" />
        <p>Inventera</p>
      </button>
    </div>
    <template v-if="data.length > 0">
      <template v-for="storage in data" :key="storage.name">
        <PanelTemplate :title="storage.name" :icon="ClipboardDocumentListIcon">
          <DynamicTree
            :rows="storage.containers || []"
            :columns="columns"
            :node="storage.containers?.[0].items[0] || {}"
          >
            <template #row="{ row, container }">
              <td class="p-2 border-b border-(--zaiko-bg-2)">
                <RouterLink
                  :to="'/item/' + encodeURIComponent(row.name)"
                  class="hover:underline"
                >
                  {{ row.name }}
                </RouterLink>
              </td>
              <td class="p-2 border-b border-(--zaiko-bg-2)">
                {{ row.amount + unitText(row.unit) }}
              </td>
              <td class="p-2 border-b border-(--zaiko-bg-2) flex">
                <input
                  type="number"
                  class="text-(--zaiko-text) max-w-15"
                  :placeholder="row.amount.toString()"
                  v-model.number="
                    new_amounts[itemKey(storage.name, container.name, row.name)]
                  "
                />
                <p class="pl-2">{{ unitText(row.unit) }}</p>
              </td>
              <td>
                <p
                  class="p-2 border-b border-(--zaiko-bg-2)"
                  :class="itemDiffColor(storage.name, container.name, row.name)"
                >
                  {{ itemDifference(storage.name, container.name, row.name) }}
                </p>
              </td>
            </template>
          </DynamicTree>
        </PanelTemplate>
      </template>
    </template>
    <div v-else>
      <div class="flex flex-col items-center justify-center h-64 gap-4">
        <span class="flex items-center gap-2">
          <HandThumbUpIcon class="w-12 h-12 text-(--zaiko-text)" />
          <p class="text-2xl text-(--zaiko-text)">
            Inga lager behöver inventeras.
          </p>
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import DynamicTree from '@/components/DynamicTree.vue'
import PanelTemplate from '@/components/PanelTemplate.vue'
import { unitText } from '@/stores/inventoryData'
import { getStockTree, takeStock } from '@/stores/stockData'
import type { StockTreeGetResponse, StockUpdateRequest } from '@/types'
import {
  ClipboardDocumentListIcon,
  HandThumbUpIcon,
  DocumentCheckIcon,
} from '@heroicons/vue/24/outline'
import { reactive, ref, computed } from 'vue'

const itemDifference = computed(() => {
  return (storage: string, container: string, name: string) => {
    const key = itemKey(storage, container, name)
    return (
      new_amounts[key] -
      (data.value
        .find(s => s.name === storage)
        ?.containers?.find(c => c.name === container)
        ?.items.find(i => i.name === name)?.amount || 0)
    )
  }
})

const itemDiffColor = computed(() => {
  return (storage: string, container: string, name: string) => {
    const diff = itemDifference.value(storage, container, name)
    if (diff > 0) return 'text-(--zaiko-main-color)'
    if (diff < 0) return 'text-(--zaiko-bad-color)'
    return ''
  }
})

const columns = {
  container: 'Låda',
  name: 'Namn',
  amount: 'Mängd',
  new_amount: 'Ny mängd',
  difference: 'Skillnad',
}
function itemKey(storage: string, container: string, name: string) {
  return `${storage}::${container}::${name}`
}

const data = ref<StockTreeGetResponse>([])
const new_amounts = reactive<Record<string, number>>({})
getStockTree().then(res => {
  data.value = res
  res.forEach(storage => {
    storage.containers?.forEach(container => {
      container.items.forEach(item => {
        const key = itemKey(storage.name, container.name, item.name)
        new_amounts[key] = item.amount
      })
    })
  })
})

const TakeStock = () => {
  const body: StockUpdateRequest = {
    items: [],
  }
  data.value.forEach(storage => {
    storage.containers?.forEach(container => {
      container.items.forEach(item => {
        const key = itemKey(storage.name, container.name, item.name)
        const input = new_amounts[key]
        if (input && input !== item.amount) {
          body.items.push({
            storage: storage.name,
            container: container.name,
            name: item.name,
            amount: input,
          })
        }
      })
    })
  })
  takeStock(body)
}
</script>

<style scoped>
.submit {
  display: flex;
  align-items: center;
  flex-direction: row;
  gap: 1rem;
}
</style>
