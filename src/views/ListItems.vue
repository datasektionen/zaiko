<template>
  <div>
    <h1>Items</h1>
    <li v-for="item in items">
      {{item.name}}: {{item.current}}
    </li>
  </div>
  <div>
    <h1>Shortage</h1>
    <li v-for="item in shortage">
      {{item.name}} amount to buy {{item.order_amount}}
    </li>
  </div>
</template>

<script setup>
import {ref} from 'vue'
const items = ref(null)
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

</style>
