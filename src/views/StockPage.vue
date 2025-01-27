<template>
  <div class="main">
    <div class="panel">
      <div class="top-bar">
        <h2 class="brist">Inventera</h2>
        <div class="top-bar filter-bar">
          <p>Filter</p>
          <input type="text">
        </div>
      </div>
      <div class="header">
        <p>Produkt</p>
        <p>Leverantör</p>
        <p>Gammla</p>
        <p>Nya</p>
        <p>Diffrens</p>
      </div>
      <form v-on:submit.prevent="updateItems" class="items">
        <StockItem v-for="item in items" :key="item.id" :item="item" />
        <div class="saveDiv">
          <input class="button" type="submit" value="Inventera" />
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import StockItem from '@/components/StockItem.vue';
import type { Item } from '@/types'
import { ref } from 'vue'
const items = ref<Array<Item>>()
const input = ref<Array<Item>>();
const HOST: string = import.meta.env.VITE_HOST;

fetch(HOST + "/api/metadorerna/item").then((res) => res.json()).then((json) => {
  items.value = json
})

const updateItems = async () => {
  // items.value?.forEach(item => {
  //   input.value?.push(item.cu) = item.current;
  // });

  await fetch(HOST + "/api/metadorerna/take_stock", {
    method: "POST",
    body: JSON.stringify(input.value),
  })
}
</script>

<style scoped>
div {
  border-radius: 2px;
  background-color: #fafafa;
  box-shadow: none;
}

.main {
  margin: 3rem;
}

.saveDiv {
  margin-top: 1rem;
  display: flex;
  align-items: flex-end;
  flex-direction: column;
}

.panel {
  width: 480px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.header {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
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
