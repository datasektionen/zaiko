<template>
  <div class="main">
    <div class="left-panel">
      <div class="top-bar">
        <button class="teal darken-1" @click="openModal = true">Lägg till</button>
        <div class="top-bar filter-bar">
          <p>Filter</p>
          <input type="text">
        </div>
      </div>
      <div class="header">
        <p>Namn</p>
        <p>Länk</p>
      </div>
      <div class="items">
        <div :class="itemSelected(idx)" v-for="(item, idx) in suppliers" :key="item.name" @click="selected = idx">
          <SupplierItem :item="item" />
        </div>
      </div>
    </div>
    <div class="left-panel" v-if="suppliers.length > 0 && selectedIndex">
      <SupplierPanel :item="selectedIndex" :key="selectedIndex.name" />
    </div>
    <PopupModal :modal="openModal" @exit="openModal = false">
      <SupplierForm @done="DoneModal()" />
    </PopupModal>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import SupplierItem from '../components/SupplierItem.vue';
import SupplierPanel from '@/components/SupplierPanel.vue';
import PopupModal from '@/components/PopupModal.vue';
import SupplierForm from '@/components/SupplierForm.vue';
import type { Supplier } from '@/types';

const HOST = import.meta.env.VITE_HOST;

const suppliers = ref<Array<Supplier>>([])

const GetData = () => {
  fetch(HOST + "/api/metadorerna/suppliers", {
    method: "GET",
  })
    .then((res) => res.json())
    .then((json) => suppliers.value = json)
}
GetData();


const DoneModal = () => {
  openModal.value = false;
  GetData()
}

const openModal = ref<boolean>(false)
const selected = ref<number>(-1);

const itemSelected = (id: number) => {
  if (id == selected.value) {
    return "item-selected"
  } else {
    return "item"
  }
}

const selectedIndex = computed<Supplier>(() => {
  return suppliers.value[selected.value];
})

</script>

<style scoped>
div {
  border-radius: 2px;
}

.main {
  display: grid;
  grid-template-columns: 50% 50%;
  gap: 1rem;
  padding: 4rem;
}

.left-panel {
  width: 480px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.item {
  border-radius: 2px;
}

.item-selected {
  border-radius: 2px;
  background-color: rgba(0, 105, 92, 0.25);
}

.header {
  display: grid;
  grid-template-columns: 1fr 1fr;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  padding: 0.5rem;
  text-align: center;
  font-weight: bold;
}

.items {
  display: flex;
  align-items: stretch;
  flex-direction: column;
}

.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.filter-bar {
  background-color: rgba(224, 242, 241, 1);
  display: flex;
  gap: 1rem;
  padding: 10px 20px;
  margin: 0;
  align-items: center;
  justify-content: space-between;
}

.filter-bar p {
  margin: 0;
  text-align: center;
}
</style>
