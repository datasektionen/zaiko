<template>
  <div class="main">
    <PanelTemplate title="Produkter" :button="permission" @button="SelectItem(-1)">
      <template #icon>
        <ArchiveBoxIcon />
      </template>
      <template #content>
        <Suspense>
          <ItemsTable @select="SelectItem" />
          <template #fallback>
            <SpinnerSimple color="#DADADA" />
          </template>
        </Suspense>
      </template>
      <template #headerRight>
        <FilterPopup :columns="columns" />
      </template>
    </PanelTemplate>
    <PopupModal :modal="isModal" @exit="UnSelect()" :title="ModalTitle">
      <Suspense>
        <ItemPanel :id="selected" @submit="UnSelect()" @delete="UnSelect()" v-if="selected != -1" />
        <ItemForm v-else @submit="UnSelect()" />
        <template #fallback>
          <SpinnerSimple color="#DADADA" />
        </template>
      </Suspense>
    </PopupModal>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { FilterColumn } from '@/types';
import PanelTemplate from '@/components/PanelTemplate.vue';
import ItemsTable from '@/components/ItemsTable.vue';
import PopupModal from '@/components/PopupModal.vue';
import ItemForm from '@/components/ItemForm.vue';
import FilterPopup from '@/components/FilterPopup.vue';
import { ArchiveBoxIcon } from '@heroicons/vue/24/outline';
import ItemPanel from '@/components/ItemPanel.vue';
import { HomeIcon, ShoppingCartIcon, WalletIcon, Battery0Icon, Battery100Icon, InformationCircleIcon } from '@heroicons/vue/16/solid';
import SpinnerSimple from '@/components/SpinnerSimple.vue';
import { useClubsStore } from '@/stores/clubs';
const clubsStore = useClubsStore();

const permission = computed(() => {
  return clubsStore.clubs.active.permission == "rw"
});
const isModal = ref<boolean>(false);
const selected = ref<number>(-1);

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
  return selected.value != -1 ? 'Redigera' : 'Lägg till';
})

const UnSelect = () => {
  selected.value = -1
  isModal.value = false
}

const SelectItem = (id: number) => {
  selected.value = id;
  isModal.value = true
}

</script>

<style scoped>
.main {
  padding: 4rem;
  padding-bottom: 0;
}

@media (max-width: 940px) {
  .main {
    padding: 2rem 0.5rem;
  }
}
</style>
