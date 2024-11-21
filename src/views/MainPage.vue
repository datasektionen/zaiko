<template>
  <div class="main">
    <div>
      <h1>Items</h1>
      <div class="header">
        <p>Produkt</p>
        <p>Plats</p>
        <p>Nämnd</p>
        <p>Min</p>
        <p>Max</p>
        <p>Nuvarande</p>
        <p>Leverantör</p>
        <p>Länk</p>
      </div>
      <div v-for="item in items" :key="item.name" class="list">
        <FrontPageItem :item="item" />
      </div>
    </div>
    <div>
      <h1>Shortage</h1>
      <div v-for="item in shortage" :key="item.name">
        <FrontPageItem :item="item" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import FrontPageItem from '../components/FrontPageItem.vue'
const items = ref([])
const shortage = ref(null)
const HOST = import.meta.env.VITE_HOST;

fetch(HOST + "/api/metadorerna/items", {
  method: "GET",
})
  .then((res) => res.json())
  .then((json) => items.value = json)

fetch(HOST + "/api/metadorerna/shortage").then((res) => res.json()).then((json) => shortage.value = json)
</script>

<style scoped>
.main {
  display: grid;
  grid-template-columns: 50% 50%;
  padding: 4rem;
}

.list {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.header {
  display: flex;
  flex-direction: row;
  gap: 1rem;
}

.header p {
  border-right: 1px solid lightgray;
  padding: 5px;
}
</style>
