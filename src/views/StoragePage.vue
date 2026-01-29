<template>
  <div class="main">
    <PanelTemplate
      :title="decodeURI(($route.params.name as string) || 'Lager')"
      :icon="ClipboardDocumentListIcon"
      :button-left-icon="FolderIcon"
      :button-left-restricted="
        !permsStore.writeAccessToStorage(
          decodeURI($route.params.name as string),
        )
      "
      @button-left="addContainer()"
      :button-right-icon="PlusIcon"
      :button-right-restricted="
        !permsStore.writeAccessToStorage(
          decodeURI($route.params.name as string),
        )
      "
      @button-right="addItem()"
    >
      <DynamicTree
        :rows="sortedContainers"
        :columns="columns"
        :node="containers[0]?.items[0] || {}"
      >
        <template #row="{ row }">
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
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            {{ row.next_inventory ? parseDate(row.next_inventory) : '-' }}
          </td>
        </template>
        <template #con="{ row }">
          <HamMenu
            :rows="settingsCol"
            v-if="
              permsStore.writeAccessToStorage(
                decodeURI(route.params.name as string),
              )
            "
            @select="
              action =>
                Settings(
                  action,
                  decodeURI(route.params.name as string),
                  row.name,
                )
            "
          />
        </template>
      </DynamicTree>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue'
import HamMenu from '@/components/HamMenu.vue'
import {
  ClipboardDocumentListIcon,
  FolderIcon,
  PlusIcon,
} from '@heroicons/vue/24/outline'
import { ref, computed, type FunctionalComponent } from 'vue'
import type {
  StorageContainersTreeGetResponse,
  StorageTreeRequest,
} from '@/types'
import { getStorageTree } from '@/stores/storageData'
import DynamicTree from '@/components/DynamicTree.vue'
import { useRoute } from 'vue-router'
import { parseDate } from '@/common'
import { unitText } from '@/stores/inventoryData'
import { usePopupStore } from '@/stores/popup'
import ContainerForm from '@/components/ContainerForm.vue'
import ItemForm from '@/components/ItemForm.vue'
import { usePermsStore } from '@/stores/permissions'
import { ArrowUpTrayIcon } from '@heroicons/vue/24/outline'
import ContainerMove from '@/components/ContainerMove.vue'
import DeleteForm from '@/components/DeleteForm.vue'
import { deleteContainer } from '@/stores/containerData'
import { BackspaceIcon } from '@heroicons/vue/24/solid'

const permsStore = usePermsStore()
const containers = ref<StorageContainersTreeGetResponse>([])

const sortedContainers = computed(() => {
  return containers.value.sort((a, b) => b.name.localeCompare(a.name))
})

const settingsCol = {
  move: 'Flytta',
  edit: 'Redigera',
  delete: 'Ta bort',
}
const Settings = (action: string, storage: string, container: string) => {
  let title = ''
  let comp = null
  let icon: FunctionalComponent = PlusIcon
  let props = {}
  let cb = undefined
  switch (action) {
    case 'edit':
      comp = ContainerForm
      props = {
        editContainer: {
          name: container,
          storage: storage,
        },
        edit: true,
      }
      icon = FolderIcon
      title = 'Redigera ' + container + ' i ' + storage
      cb = containerGhostEdit
      break
    case 'move':
      comp = ContainerMove
      props = {
        container: {
          name: container,
          storage: storage,
        },
        moveFunc: () => {},
      }
      icon = ArrowUpTrayIcon
      title = 'Flytta ' + container + ' från ' + storage
      cb = containerGhostMove
      break
    case 'delete':
      comp = DeleteForm
      title = 'Ta bort ' + container + ' från ' + storage
      props = {
        item: {
          name: container,
          storage: storage,
        },
        deleteFunc: deleteContainer,
      }
      icon = BackspaceIcon
      cb = async (result: any) => {
        if (result) {
          getStorageTree(body).then(data => {
            containers.value = data
          })
        }
      }
      break
    default:
      console.error('Unknown action:', action, container)
      break
  }
  if (comp) {
    popupStore.push({
      title: title,
      component: comp,
      props: props,
      icon: icon,
      cb: cb,
    })
  }
}

const popupStore = usePopupStore()
function addContainer() {
  popupStore.push({
    title: 'Lägg till låda',
    icon: FolderIcon,
    component: ContainerForm,
    cb: containerGhost,
    props: {
      editContainer: {
        storage: decodeURI(route.params.name as string),
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
    getStorageTree(body).then(data => {
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

function containerGhostEdit(result?: any) {
  if (result) {
    containers.value.push({
      name: result.new_name,
      items: [],
    })
  }
}

function containerGhostMove(result?: any) {
  if (result) {
    getStorageTree(body).then(data => {
      containers.value = data
    })
  }
}

const route = useRoute()
const body: StorageTreeRequest = {
  name: decodeURI(route.params.name as string),
}
getStorageTree(body).then(data => {
  containers.value = data
})

const columns = {
  container: 'Låda',
  name: 'Namn',
  amount: 'Mängd',
  next_inventory: 'Nästa inventering',
}
</script>

<style scoped></style>
