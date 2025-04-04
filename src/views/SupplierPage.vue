<template>
  <div class="main">
    <PanelTemplate title="Leverantörer" button @button="SelectItem(-1)">
      <template #icon>
        <ShoppingCartIcon />
      </template>
      <template #content>
        <Suspense>
          <SupplierTable @select="SelectItem" />
          <template #fallback>
            <p>Laddar...</p>
          </template>
        </Suspense>
      </template>
      <template #headerRight>
        <FilterPopup :columns="columns" />
      </template>
    </PanelTemplate>
    <PopupModal :modal="isModal" @exit="UnSelect()" :title="ModalTitle">
      <Suspense>
        <SupplierPanel v-if="selected != -1" :id="selected" @submit="UnSelect()" @delete="UnSelect()" />
        <SupplierForm v-else @submit="UnSelect()" />
        <template #fallback>
          <p>Laddar...</p>
        </template>
      </Suspense>
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
import type { FilterColumn } from '@/types';
import { ShoppingCartIcon as ShoppingCartIconSmall, UserCircleIcon, LockClosedIcon, DocumentTextIcon } from '@heroicons/vue/16/solid';
import FilterPopup from '@/components/FilterPopup.vue';

const isModal = ref<boolean>(false);
const selected = ref<number>(-1);

const ModalTitle = computed(() => {
  return selected.value != -1 ? 'Redigera' : 'Lägg till';
})

const columns: Array<FilterColumn> = [
  { name: 'name', label: 'Namn', icon: ShoppingCartIconSmall },
  { name: 'username', label: 'Användarnamn', icon: UserCircleIcon },
  { name: 'password', label: 'Lösenord', icon: LockClosedIcon },
  { name: 'notes', label: 'Anteckningar', icon: DocumentTextIcon },
];

const UnSelect = () => {
  selected.value = -1
  isModal.value = false
}

const SelectItem = (id: number) => {
  selected.value = id
  isModal.value = true
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
