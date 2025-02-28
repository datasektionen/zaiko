<template>
  <div class="main">
    <PanelTemplate title="Produkter" button @button="SelectItem(-1)">
      <template #icon>
        <ArchiveBoxIcon />
      </template>
      <template #content>
        <ItemsTable :items="items" @select="SelectItem" />
      </template>
      <template #headerRight>
        <FilterPopup :columns="columns" @search="Filter" @clear="GetData()" />
      </template>
    </PanelTemplate>
    <PopupModal :modal="isModal" @exit="UnSelect()" :title="ModalTitle">
      <Suspense>
        <ItemPanel :item="selected" @submit="GetData()" @delete="GetData()" v-if="selected != undefined" />
        <ItemForm v-else @submit="GetData()" />
      </Suspense>
    </PopupModal>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { ItemGetResponse, FilterItemParams, Notification, FilterColumn } from '@/types';
import PanelTemplate from '@/components/PanelTemplate.vue';
import ItemsTable from '@/components/ItemsTable.vue';
import PopupModal from '@/components/PopupModal.vue';
import ItemForm from '@/components/ItemForm.vue';
import FilterPopup from '@/components/FilterPopup.vue';
import { ArchiveBoxIcon } from '@heroicons/vue/24/outline';
import ItemPanel from '@/components/ItemPanel.vue';
import { HomeIcon, ShoppingCartIcon, WalletIcon, Battery0Icon, Battery100Icon, InformationCircleIcon } from '@heroicons/vue/16/solid';
import { useClubsStore } from '@/stores/clubs';
import { useNotificationsStore } from '@/stores/notifications';
const HOST = import.meta.env.VITE_HOST;

const items = ref<Array<ItemGetResponse>>([]);
const isModal = ref<boolean>(false);
const selected = ref<ItemGetResponse>();

const columns: Array<FilterColumn> = [
  { name: 'product', label: 'Produkt', icon: ArchiveBoxIcon },
  { name: 'location', label: 'Plats', icon: HomeIcon },
  { name: 'supplier', label: 'Leverantör', icon: ShoppingCartIcon },
  { name: 'amount', label: 'Mängd', icon: WalletIcon },
  { name: 'min', label: 'Min', icon: Battery0Icon },
  { name: 'max', label: 'Max', icon: Battery100Icon },
  { name: 'status', label: 'Status', icon: InformationCircleIcon },
];

const ModalTitle = computed(() => {
  return selected.value ? 'Redigera' : 'Lägg till';
})

const UnSelect = () => {
  selected.value = undefined
  isModal.value = false
}

const SelectItem = (idx: number) => {
  selected.value = items.value[idx]
  isModal.value = true
}


const notificationsStore = useNotificationsStore();
const clubStore = useClubsStore();

const GetData = () => {
  if (!clubStore.checkClub()) return
  UnSelect();
  const url: string = HOST + "/api/" + clubStore.getClub();
  fetch(url + "/item", {
    method: "GET",
  }).then((r) => r.json()).then((r) => items.value = r)
    .catch((error) => {
      const noti: Notification = {
        id: Date.now(),
        title: "Error",
        message: error.toString(),
        severity: "error",
      }
      notificationsStore.add(noti);
    })

}
GetData()

const Filter = (column: string, search: string) => {
  if (!clubStore.checkClub()) return
  const url: string = HOST + "/api/" + clubStore.getClub() + "/item?";
  const query: FilterItemParams = {
    column: column,
    search: search,
  }
  const queryString = new URLSearchParams(Object.entries(query)).toString();
  fetch(url + queryString, {
    method: "GET",
  }).then((r) => r.json()).then((r) => items.value = r)
    .catch((error) => {
      const noti: Notification = {
        id: Date.now(),
        title: "Error",
        message: error.toString(),
        severity: "error",
      }
      notificationsStore.add(noti);
    })
}
</script>

<style scoped>
.main {
  padding: 4rem;
  padding-bottom: 0;
}

@media (max-width: 768px) {
  .main {
    padding: 0.3rem;
  }
}
</style>
