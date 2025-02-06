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
        <p>Produkt</p>
        <p>Plats</p>
        <p>Mängd</p>
        <p>Status</p>
      </div>
      <div class="items">
        <FrontPageItem :item="item" v-for="item in items" :key="item.id" />
      </div>
    </div>
    <div class="left-panel">
      <div class="top-bar">
        <h2 class="brist">Brist</h2>
        <div class="top-bar filter-bar">
          <p>Filter</p>
          <input type="text">
        </div>
      </div>
      <div class="header">
        <p>Produkt</p>
        <p>Leverantör</p>
        <p>Mängd</p>
        <p>Att köpa</p>
      </div>
      <div class="items">
        <FrontPageShortageItem :item="item" v-for="item in shortage" :key="item.name" />
      </div>
    </div>
    <PopupModal :modal="openModal" @exit="openModal = false">
      <AddForm @done="DoneModal()" />
    </PopupModal>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import FrontPageItem from '@/components/FrontPageItem.vue'
import PopupModal from '@/components/PopupModal.vue'
import AddForm from '@/components/AddForm.vue'
import FrontPageShortageItem from '@/components/FrontPageShortageItem.vue';
import type { ItemGetResponse, ShortageItem } from '@/types';

const HOST = import.meta.env.VITE_HOST;
const items = ref<Array<ItemGetResponse>>();
const shortage = ref<Array<ShortageItem>>()

const openModal = ref<boolean>(false)

const DoneModal = () => {
  openModal.value = false;
  GetData();
}

const GetData = () => {
  fetch(HOST + "/api/metadorerna/item", {
    method: "GET",
  })
    .then((res) => res.json())
    .then((json) => items.value = json)

  fetch(HOST + "/api/metadorerna/shortage")
    .then((res) => res.json())
    .then((json) => shortage.value = json)

}
GetData();
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

.header {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr;
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

.brist {
  padding-bottom: 1rem;
  margin: 0;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);

}
</style>
