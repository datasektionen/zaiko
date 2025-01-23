<template>
  <div class="main">
    <div>
      <h1>Produkter</h1>
      <div class="itemGrid">
        <div class="header">
          <p>Produkt</p>
          <p>Plats</p>
          <p>Nämnd</p>
          <p>Min</p>
          <p>Max</p>
          <p>Nuvarande</p>
          <p>Leverantör</p>
          <p>Länk</p>
          <p>Edit</p>
        </div>
        <div v-for="item in items" :key="item.name" class="Items">
          <FrontPageItem :item="item" />
        </div>
      </div>
    </div>
    <div>
      <h1>Brist</h1>
      <div class="itemGrid">
        <div class="header">
          <p>Produkt</p>
          <p>Plats</p>
          <p>Nämnd</p>
          <p>Min</p>
          <p>Max</p>
          <p>Nuvarande</p>
          <p>Leverantör</p>
          <p>Länk</p>
          <p>Edit</p>
        </div>
        <div v-for="item in shortage" :key="item.name">
          <FrontPageItem :item="item" />
        </div>
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

fetch(HOST + "/api/metadorerna/item", {
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
  gap: 1rem;
  padding: 4rem;
}

.itemGrid {
  display: grid;
  grid-auto-flow: row;
  overflow-x: scroll;
}

.list {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.header {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
}

.header p {
  border-right: 1px solid lightgray;
  padding: 1rem;
}
</style>
