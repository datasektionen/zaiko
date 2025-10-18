<template>
  <div class="main">
    <PanelTemplate :title="$route.params.name as string || 'Lager'" :icon="ClipboardDocumentListIcon"
      :button-left-icon="FolderIcon"
      :button-left-restricted="!permsStore.writeAccessToStorage($route.params.name as string)"
      @button-left="addContainer()" :button-right-icon="PlusIcon"
      :button-right-restricted="!permsStore.writeAccessToStorage($route.params.name as string)"
      @button-right="addItem()">
      <DynamicTree :rows="containers" :columns="columns" checkbox>
        <template #row="input">
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <RouterLink :to="'/item/' + input.row.name" class="hover:underline">
              {{ input.row.name }}
            </RouterLink>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">{{ input.row.amount + unitText(input.row.unit as string) }}
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">{{ parseDate(input.row.next_inventory as string) }}</td>
        </template>
      </DynamicTree>
    </PanelTemplate>

  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue'
import { ClipboardDocumentListIcon, FolderIcon, PlusIcon } from '@heroicons/vue/24/outline'
import { ref } from 'vue'
import type { StorageContainersTreeGetResponse, StorageTreeGetResponse, StorageTreeRequest } from '@/types'
import { getStorageTree } from '@/stores/storageData'
import DynamicTree from '@/components/DynamicTree.vue'
import { useRoute } from 'vue-router'
import { parseDate } from '@/common'
import { unitText } from '@/stores/inventoryData'
import { usePopupStore } from '@/stores/popup'
import ContainerForm from '@/components/ContainerForm.vue'
import ItemForm from '@/components/ItemForm.vue'
import { usePermsStore } from '@/stores/permissions'

const permsStore = usePermsStore()
const containers = ref<StorageContainersTreeGetResponse>([])

const popupStore = usePopupStore()
function addContainer() {
  popupStore.push({
    title: 'Lägg till låda',
    icon: FolderIcon,
    component: ContainerForm,
    cb: containerGhost,
    props: {
      editContainer: {
        storage: route.params.name as string,
      },
    },
  })
}

const addItem = () => {
  popupStore.push({
    title: 'Lägg till produkt',
    component: ItemForm,
    icon: PlusIcon,
    props: {
      editItem: {
        storage: route.params.name as string,
        container: (route.query.container as string) || undefined,
      },
    },
    cb: addItemGhost,
  })
}

function addItemGhost(result?: any) {
  if (result) {
    getStorageTree(body).then((data) => {
      containers.value = data
    })
  }
}

function containerGhost(result?: any) {
  if (result) {
    containers.value.push({
      name: result.name,
      items: [],
    })
  }
}

const route = useRoute()
const body: StorageTreeRequest = {
  name: route.params.name as string
}
getStorageTree(body).then((data) => {
  containers.value = data
})

const columns = {
  container: 'Låda',
  name: 'Namn',
  amount: 'Mängd',
  next_inventory: 'Nästa inventering'
}

</script>

<style scoped></style>
