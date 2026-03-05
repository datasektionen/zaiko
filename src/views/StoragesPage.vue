<template>
  <div class="main">
    <PanelTemplate title="Lager" :icon="InboxIcon">
      <DynamicTable :rows="rows" :columns="columns">
        <template #row="input">
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <RouterLink :to="'/storage/' + encodeURIComponent(input.row.name)">
              <p class="hover:underline">{{ input.row.name }}</p>
            </RouterLink>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <span
              v-for="container in input.row.containers"
              :key="container"
              class="mr-2 inline-block"
            >
              <RouterLink
                :to="
                  '/storage/' +
                  encodeURIComponent(input.row.name) +
                  '?container=' +
                  encodeURIComponent(container)
                "
              >
                <p class="hover:underline">{{ container }}</p>
              </RouterLink>
            </span>
          </td>
        </template>
      </DynamicTable>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import PanelTemplate from '@/components/PanelTemplate.vue'
import { InboxIcon } from '@heroicons/vue/24/outline'
import type { StorageContainersGetResponse } from '@/types'
import DynamicTable from '@/components/DynamicTable.vue'
import { getStorageContainers } from '@/stores/storageData'

const columns = {
  name: 'Namn',
  container: 'Lådor',
}

const rows = ref<StorageContainersGetResponse>([])
getStorageContainers().then(data => {
  rows.value = data
})
</script>

<style scoped></style>
