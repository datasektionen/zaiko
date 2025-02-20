<template>
  <div class="main">
    <PanelTemplate title="Leverantörer" button @button="SelectItem(-1)">
      <template #icon>
        <ShoppingCartIcon />
      </template>
      <template #content>
        <SupplierTable :items="suppliers" @select="SelectItem" />
      </template>
      <template #headerRight>
        <FilterPopup :columns="columns" @search="Filter" @clear="GetData()"/>
      </template>
    </PanelTemplate>
    <PopupModal :modal="isModal" @exit="UnSelect()" :title="ModalTitle">
      <SupplierPanel v-if="selected" :item="selected" @submit="GetData()" @delete="GetData()"/>
      <SupplierForm v-else @submit="GetData()"/>
    </PopupModal>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import PanelTemplate from '@/components/PanelTemplate.vue';
import SupplierTable from '@/components/SupplierTable.vue';
import PopupModal from '@/components/PopupModal.vue';
import SupplierForm from '@/components/SupplierForm.vue';
import { ShoppingCartIcon } from '@heroicons/vue/24/outline';
import SupplierPanel from '@/components/SupplierPanel.vue';
import type { SupplierGetResponse, Notification, FilterItemParams, FilterColumn } from '@/types';
import { useNotificationsStore } from '@/stores/notifications';
import { useClubsStore } from '@/stores/clubs';
import { ShoppingCartIcon as ShoppingCartIconSmall, UserCircleIcon, LockClosedIcon, DocumentTextIcon } from '@heroicons/vue/16/solid';
import FilterPopup from '@/components/FilterPopup.vue';

const HOST = import.meta.env.VITE_HOST;

const suppliers = ref<Array<SupplierGetResponse>>([]);
const isModal = ref<boolean>(false);
const selected = ref<SupplierGetResponse>();

const ModalTitle = computed(() => {
  return selected.value ? 'Redigera' : 'Lägg till';
})

const columns: Array<FilterColumn> = [
  { name: 'name', label: 'Namn', icon: ShoppingCartIconSmall },
  { name: 'username', label: 'Användarnamn', icon: UserCircleIcon },
  { name: 'password', label: 'Lösenord', icon: LockClosedIcon },
  { name: 'notes', label: 'Anteckningar', icon: DocumentTextIcon },
];

const UnSelect = () => {
  selected.value = undefined
  isModal.value = false
}

const SelectItem = (idx: number) => {
  selected.value = suppliers.value[idx]
  isModal.value = true
}

const notificationsStore = useNotificationsStore();
const clubStore = useClubsStore();

const GetData = () => {
  const url: string = HOST + "/api/" + clubStore.getClub();

  fetch(url + "/supplier", {
    method: "GET",
  })
    .then((res) => res.json())
    .then((json) => suppliers.value = json)
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
GetData();

const Filter = (column: string, search: string) => {
  const url: string = HOST + "/api/" + clubStore.getClub() + "/supplier?";
  const query: FilterItemParams = {
    column: column,
    search: search,
  }
  const queryString = new URLSearchParams(Object.entries(query)).toString();
  fetch(url + queryString, {
    method: "GET",
  }).then((r) => r.json()).then((r) => suppliers.value = r)
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
    padding: 0.4rem;
  }
}
</style>
