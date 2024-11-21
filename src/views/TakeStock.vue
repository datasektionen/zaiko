<template>
  <div>
    <form v-on:submit.prevent="updateItems">
      <li v-for="item in items" :key="item.name">
        {{ item.name }} {{ item.current }} <input v-model.number="input[item.name]" /> {{ item.current - input[item.name] }}
      </li>
      <input type="submit" />
    </form>
  </div>
  <button></button>
</template>

<script setup lang="ts">
  import {ref, reactive} from 'vue'
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

</style>
