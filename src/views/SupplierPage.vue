<template>
  <div class="main">
    <PanelTemplate title="Leverantörer" button @button="SelectItem(-1)">
      <template #icon>
        <ShoppingCartIcon />
      </template>
      <template #content>
        <SupplierTable :items="items" @select="SelectItem" />
      </template>
    </PanelTemplate>
    <PopupModal :modal="isModal" @exit="UnSelect()" :title="ModalTitle">
      <SupplierPanel v-if="selected" :item="selected" />
      <SupplierForm v-else />
    </PopupModal>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { SupplierGetResponse } from '@/types';
import PanelTemplate from '@/components/PanelTemplate.vue';
import SupplierTable from '@/components/SupplierTable.vue';
import PopupModal from '@/components/PopupModal.vue';
import SupplierForm from '@/components/SupplierForm.vue';
import { ShoppingCartIcon } from '@heroicons/vue/24/outline';
import SupplierPanel from '@/components/SupplierPanel.vue';

const items = ref<Array<SupplierGetResponse>>([]);
const isModal = ref<boolean>(false);
const selected = ref<SupplierGetResponse>();

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
      name: "IKEA",
      link: "https://www.ikea.se",
      updated: 100,
      notes: "Leverantör av möbler och inredning",
      password: "password",
      username: "username",
    },
    {
      id: 1,
      name: "Bauhaus",
      link: "https://www.bauhaus.se",
      updated: 100,
      notes: "Leverantör av byggmaterial",
      password: "password",
      username: "username",
    },
    {
      id: 2,
      name: "K-rauta",
      link: "https://www.k-rauta.se",
      updated: 100,
      notes: "Leverantör av byggmaterial",
      password: "password",
      username: "username",
    },
    {
      id: 3,
      name: "Clas Ohlson",
      link: "https://www.clasohlson.com/se",
      updated: 100,
      notes: "Leverantör av verktyg och hushållsprodukter",
      password: "password",
      username: "username",
    },
    {
      id: 4,
      name: "Jula",
      link: "https://www.jula.se",
      updated: 100,
      notes: "Leverantör av verktyg och hushållsprodukter",
      password: "password",
      username: "username",
    },
    {
      id: 5,
      name: "Hornbach",
      link: "https://www.hornbach.se",
      updated: 100,
      notes: "Leverantör av byggmaterial",
      password: "password",
      username: "username",
    },
    {
      id: 6,
      name: "Byggmax",
      link: "https://www.byggmax.se",
      updated: 100,
      notes: "Leverantör av byggmaterial",
      password: "password",
      username: "username",
    },
    {
      id: 7,
      name: "Biltema",
      link: "https://www.biltema.se",
      updated: 100,
      notes: "Leverantör av verktyg och hushållsprodukter",
      password: "password",
      username: "username",
    },
    {
      id: 8,
      name: "XL Bygg",
      link: "https://www.xlbygg.se",
      updated: 100,
      notes: "Leverantör av"
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
