<template>
  <div class="main">
      <div v-for="supplier in suppliers" :key="supplier.name">
        <SupplierItem :item="supplier" />
      </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import SupplierItem from '../components/SupplierItem.vue';

// Probebly not a good idea to use an object, due to security, should probebly use map
const HOST = import.meta.env.VITE_HOST;

const suppliers = ref([])

fetch(HOST + "/api/metadorerna/suppliers", {
  method: "GET",
})
  .then((res) => res.json())
  .then((json) => suppliers.value = json)

</script>

<style scoped>
.main {
  max-width: 1024px;
  margin: 1rem auto;
}
</style>
