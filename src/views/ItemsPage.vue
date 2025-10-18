<template>
  <div class="pt-2 md:pt-8 md:p-3">
    <PanelTemplate title="Produkter" :icon="ArchiveBoxIcon" :buttonLeftIcon="PlusIcon"
      :buttonLeftRestricted="!permsStore.hasWriteAccess()" @buttonLeft="addItem()">
      <SearchBar @send="(f) => filter = f" :storages="storages" :suppliers="suppliers" />
      <DynamicTable :rows="rows" :columns="columns">
        <template #row="input">
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <RouterLink :to="'/item/' + encodeURIComponent(input.row['name'] as string)">
              <p class="hover:underline">{{ input.row['name'] }}</p>
            </RouterLink>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <span v-for="storage in input.row.storage" :key="storage.toString()" class="mr-2 inline-block">
              <RouterLink :to="storagePath(storage.storage, storage.container)">
                <p :class="'hover:underline ' + stateColor(storage.state)">{{ storage.storage }}{{ containerText(storage.container) }}</p>
              </RouterLink>
            </span>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <p>{{ input.row['amount'] }}</p>
          </td>
        </template>
      </DynamicTable>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import type { ItemListGetResponse, ItemListQueryParams, StorageContainersGetResponse, SupplierGetResponse} from '@/types';
import PanelTemplate from '@/components/PanelTemplate.vue';
import { ArchiveBoxIcon, PlusIcon } from '@heroicons/vue/24/outline';
import { getItems } from '@/stores/itemData';
import DynamicTable from '@/components/DynamicTable.vue';
import { RouterLink } from 'vue-router';
import SearchBar from '@/components/SearchBar.vue';
import { getStorageContainers, getStorages } from '@/stores/storageData';
import { usePopupStore } from '@/stores/popup';
import ItemForm from '@/components/ItemForm.vue';
import { containerText } from '@/stores/inventoryData';
import { usePermsStore } from '@/stores/permissions';
import { getSuppliers } from '@/stores/supplierData';
import { stateColor, stateEmoji } from '@/common';

const popupStore = usePopupStore();
const permsStore = usePermsStore();

function storagePath(storage: string, container?: string) {
  return '/storage/' + encodeURIComponent(storage) + (container ? '?container=' + encodeURIComponent(container) : '');
}

function addItemGhost(result?: any) {
  if (result) {
    getItems(filter.value).then((data) => {
      rows.value = data;
    });
  }
}

const addItem = () => {
  popupStore.push({
    title: 'Lägg till produkt',
    component: ItemForm,
    icon: PlusIcon,
    cb: addItemGhost,
  })
}

const filter = ref<ItemListQueryParams>({});
watch(filter, (newFilter) => {
  getItems(newFilter).then((data) => {
    rows.value = data;
  });
}, { deep: true });

const columns = {
  name: 'Namn',
  storage: 'Lager',
  amount: 'Mängd',
};

const storages = ref<StorageContainersGetResponse>([]);
getStorageContainers().then((data) => {
  storages.value = data;
});

const rows = ref<ItemListGetResponse>([]);
getItems(filter.value).then((data) => {
  rows.value = data;
});

const suppliers = ref<SupplierGetResponse>([]);
getSuppliers().then((data) => {
  suppliers.value = data;
});

</script>

<style scoped></style>
