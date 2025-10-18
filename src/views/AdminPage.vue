<template>
  <div>
    <PanelTemplate title="Admin Panel" :icon="WrenchIcon" :button-left-icon="PlusIcon" @button-left="addStorage()">
      <DynamicTable :rows="storages" :columns="columns" settings>
        <template #row="input">
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <RouterLink :to="'/storage/' + encodeURIComponent(input.row.name)">
              <p class="hover:underline">{{ input.row.name }}</p>
            </RouterLink>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <p>{{ Interval(input.row.inventory_interval) }}</p>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <component :is="locked(input.row.protected)" class="w-5 h-5 text-(--zaiko-accent)"
              v-if="input.row.protected" />
          </td>
        </template>
        <template #settings="input">
          <HamMenu :rows="settings.props.rows" @select="(action) => Settings(action, input.row.name)" />
        </template>
      </DynamicTable>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue';
import DynamicTable from '@/components/DynamicTable.vue';
import { LockClosedIcon, WrenchIcon, PlusIcon, PencilSquareIcon, BackspaceIcon } from '@heroicons/vue/24/outline';
import type { StorageContainersGetResponse, StoragesGetResponse } from '@/types';
import { markRaw, ref, type FunctionalComponent } from 'vue';
import { durationToReadableString, parseISODuration } from '@/common';
import { deleteStorage, getStorageContainers, getStorages } from '@/stores/storageData';
import { usePopupStore } from '@/stores/popup';
import StorageForm from '@/components/StorageForm.vue';
import HamMenu from '@/components/HamMenu.vue';
import DeleteForm from '@/components/DeleteForm.vue';

const Interval = (interval?: string | null): string => {
  if (!interval) return "";
  const timeLeft = durationToReadableString(parseISODuration(interval));
  return timeLeft;
}
const popupStore = usePopupStore();
const storages = ref<StoragesGetResponse>([]);

const settings = {
  component: markRaw(HamMenu),
  props: {
    rows: { edit: 'Redigera', delete: 'Ta bort' },
  },
}

const Settings = (action: string, item: string) => {
  let title = "";
  let comp = null;
  let icon: FunctionalComponent = PlusIcon;
  let props = {};
  let cb = undefined;
  switch (action) {
    case 'edit':
      comp = StorageForm;
      title = "Redigera " + item;
      props = {
        edit: true,
        editStorage: storages.value.find((s) => s.name === item)
      };
      icon = PencilSquareIcon
      cb = editStorageGhost
      break;
    case 'delete':
      comp = DeleteForm;
      title = "Är du säker?";
      props = {
        item: { name: item },
        deleteFunc: deleteStorage
      };
      icon = BackspaceIcon
      cb = removeStorageGhost
      break;
    default:
      console.error("Unknown action:", action, item);
      break;
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

function removeStorageGhost(result?: any) {
  if (result) {
    console.log("Removing storage from list:", result);
    const index = storages.value.findIndex((s) => s.name === result.name);
    if (index !== -1) {
      storages.value.splice(index, 1);
    }
  }
}

function editStorageGhost(result?: any) {
  if (result) {
    console.log("Editing storage in list:", result);
    const index = storages.value.findIndex((s) => s.name === result.name);
    if (index !== -1) {
      storages.value[index] = result;
    }
  }
}

function addStorageGhost(result?: any) {
  if (result) {
    console.log("Adding new storage to list:", result);
    storages.value.push(result);
  }
}

const addStorage = () => {
  popupStore.push({
    title: 'Skapa ett Lager',
    component: StorageForm,
    icon: PlusIcon,
    cb: addStorageGhost,
  })
}

getStorages().then((data) => {
  storages.value = data;
});

const locked = (locked: boolean) => {
  return locked ? LockClosedIcon : null;
}

const columns = {
  name: 'Namn',
  next_inventory: 'Intervall',
  protected: 'Skyddad',
}

</script>

<style scoped></style>
