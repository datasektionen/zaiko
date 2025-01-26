<template>
  <div class="main">
    <h1>Lägg till</h1>
    <form v-on:submit.prevent="addItem">
      <div class="item">
        <p>Produkt</p>
        <input v-model="name" placeholder="Produkt">
      </div>
      <div class="item">
        <p>Plats</p>
        <input v-model="location" placeholder="Plats">
      </div>
      <fieldset>
        <div class="item">
          <p>Min</p>
          <input type="number" v-model="min" placeholder="Min">
        </div>
        <div class="item">
          <p>Max</p>
          <input type="number" v-model="max" placeholder="Max">
        </div>
        <div class="item">
          <p>Nuvarande</p>
          <input type="number" v-model="current" placeholder="Nuvarande">
        </div>
      </fieldset>
      <div class="item">
        <p>Leverantör</p>
        <input v-model="supplier" placeholder="Leverantör">
      </div>
      <div class="item">
        <p>Länk</p>
        <input v-model="link" placeholder="Länk">
      </div>
      <input v-model="club" placeholder="Nämnd">
      <div class="submit">
        <input class="button" type="submit" value="Lägg till">
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import router from '@/router';
import type { AddItem } from '@/types';
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
  const res: AddItem = {
    name: name.value,
    location: location.value,
    min: min.value,
    max: max.value,
    current: current.value,
    supplier: Number.parseInt(supplier.value),
    link: link.value,
  }
  fetch(HOST + "/api/" + club.value + "/item", {
    method: "POST",
    body: JSON.stringify(res),
  })
    .then(() => router.push("/"))
}

</script>

<style scoped>
.main {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: flex-start;
  gap: 2rem;
  max-width: 1024px;
  margin: 2rem auto;
}

p {
  margin: 0;
}

h1 {
  padding-bottom: 10px;
  margin: 0;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  width: auto;
}

form {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  gap: 2rem;
}

fieldset {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 3rem;
}

.item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  padding: 8px 0;
  margin-bottom: 8px;
}

.item input {
  border: none;
  background-color: inherit;
  text-align: right;
}

.submit {
  display: flex;
  align-items: flex-end;
  flex-direction: column;
}
</style>
