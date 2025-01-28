<template>
  <div>
    <form v-on:submit.prevent="addItem">
      <input v-model="club" placeholder="Nämnd">
      <input v-model="name" placeholder="Produkt">
      <input v-model="location" placeholder="Plats">
      <fieldset>
        <label>
          Min:
          <input type="number" v-model="min" placeholder="min">
        </label>
        <label>
          Max:
          <input type="number" v-model="max" placeholder="max">
        </label>
        <label>
          Nuvarande:
          <input type="number" v-model="current" placeholder="nuvarande">
        </label>
      </fieldset>
      <input v-model="supplier" placeholder="Leverantör">
      <input v-model="link" placeholder="Länk">
      <input type="submit" value="Lägg till">
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
const HOST = import.meta.env.VITE_HOST;

const club = ref("")
const name = ref("")
const location = ref("")
const min = ref(0)
const max = ref(0)
const current = ref(0)
const supplier = ref("")
const link = ref("")

const addItem = async () => {
  const res = {
    name: name.value,
    location: location.value,
    min: min.value,
    max: max.value,
    current: current.value,
    supplier: supplier.value,
    link: link.value,
  }
  await fetch(HOST + "/api/" + club.value + "/item", {
    method: "POST",
    body: JSON.stringify(res),
  })
}

</script>

<style scoped>
form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  max-width: 1024px;
  margin: 2rem auto;
}

input {}
</style>
