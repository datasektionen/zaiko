<template>
  <div>
    <li v-for="item in data">
      {{item.name}}: {{item.current}}
    </li>
  </div>
</template>

<script setup>
import {ref} from 'vue'
const data = ref(null)
const HOST = import.meta.env.VITE_HOST;

fetch(HOST + "/api/" + "metadorerna" + "/items", {
  method: "GET",
})
.then((res) => res.json())
  .then((json) => data.value = json)
</script>

<style scoped>

</style>
