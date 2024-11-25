<template>
  <div class="main">
    <div class="header">
      <p>Produkt</p>
      <p>Plats</p>
      <p>Nuvarande</p>
      <p>Nya</p>
      <p>Differens</p>
    </div>
    <form v-on:submit.prevent="updateItems">
      <div v-for="item in items" :key="item.name">
        <StockItem :item="item" />
      </div>
      <input type="submit" value="Inventera" />
    </form>
  </div>
</template>

<script setup lang="ts">
import StockItem from '@/components/StockItem.vue';
import { ref, reactive } from 'vue'
const items = ref(null)
// Probebly not a good idea to use an object, due to security, should probebly use map
const input = reactive({})
const HOST = import.meta.env.VITE_HOST;

fetch(HOST + "/api/metadorerna/items").then((res) => res.json()).then((json) => {
  items.value = json
  json.forEach(item => {
    input[item.name] = 0
  });
})

const updateItems = async () => {
  await fetch(HOST + "/api/metadorerna/take_stock", {
    method: "POST",
    body: JSON.stringify(Object.entries(input)),
  })
}
</script>

<style scoped>
.main {
  max-width: 1024px;
  margin: 1rem auto;
}

.header {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
}

.header p,
a {
  border-right: 1px solid lightgray;
  padding: 1rem;
}
</style>
