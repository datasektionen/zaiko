<template>
  <div class="main">
    <PanelTemplate title="Produkter" button @button="SelectItem(-1)">
      <template #icon>
        <ArchiveBoxIcon />
      </template>
      <template #content>
        <ItemsTable :items="items" @select="SelectItem" />
      </template>
    </PanelTemplate>
    <PopupModal :modal="isModal" @exit="UnSelect()" :title="ModalTitle">
      <ItemPanel v-if="selected" :item="selected" />
      <ItemForm v-else />
    </PopupModal>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { ItemGetResponse } from '@/types';
import PanelTemplate from '@/components/PanelTemplate.vue';
import ItemsTable from '@/components/ItemsTable.vue';
import PopupModal from '@/components/PopupModal.vue';
import ItemForm from '@/components/ItemForm.vue';
import { ArchiveBoxIcon } from '@heroicons/vue/24/outline';
import ItemPanel from '@/components/ItemPanel.vue';

const items = ref<Array<ItemGetResponse>>([]);
const isModal = ref<boolean>(false);
const selected = ref<ItemGetResponse>();

const ModalTitle = computed(() => {
  return selected.value ? 'Redigera' : 'Lägg till';
})

const UnSelect = () => {
  selected.value = undefined
  isModal.value = false
}

const SelectItem = (idx: number) => {
  console.log('Selected item:', items.value[idx])
  selected.value = items.value[idx]
  isModal.value = true
}


const GetData = () => {
  items.value = [
    {
      id: 0,
      name: 'Telp',
      supplier: 0,
      link: 'https://www.telp.se',
      current: 10,
      min: 5,
      max: 20,
      location: 'A1',
      updated: 100,
    },
    {
      id: 1,
      name: 'Telp',
      supplier: 2,
      link: 'https://www.telp.se',
      current: 10,
      min: 5,
      max: 20,
      location: 'A1',
      updated: 100,
    },
    {
      id: 2,
      name: 'Telp',
      supplier: 0,
      link: 'https://www.telp.se',
      current: 10,
      min: 5,
      max: 20,
      location: 'A1',
      updated: 100,
    },
  ]
}
GetData()

</script>

<style scoped>
.main {
  padding: 4rem;
  padding-bottom: 0;
}
</style>
