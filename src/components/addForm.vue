<template>
  <div>
    <form v-on:submit.prevent="addItem">
      <input v-model="club" placeholder="club">
      <input v-model="name" placeholder="name">
      <input v-model="location" placeholder="locatian">
      <input type="number" v-model="min" placeholder="min">
      <input type="number" v-model="max" placeholder="max">
      <input type="number" v-model="current" placeholder="current">
      <input type="submit">
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
const HOST = import.meta.env.VITE_HOST;

const club: string = ref("")
const name: string = ref("")
const location: string = ref("")
const min: number = ref(0)
const max: number = ref(0)
const current: number = ref(0)

const addItem = async () => {
  const res = {
    name: name.value,
    location: location.value,
    min: min.value,
    max: max.value,
    current_amount: current.value,

  }
  await fetch(HOST + "/api/" + club.value + "/item", {
    method: "POST",
    body: JSON.stringify(res),
  })
}

</script>

<style scoped></style>
