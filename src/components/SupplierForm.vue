<template>
  <div>
    <form v-on:submit.prevent="addItem">
      <input v-model="name" placeholder="Namn">
      <input v-model="link" placeholder="Länk">
      <input v-model="username" placeholder="Användarnamn">
      <input v-model="password" placeholder="Lösenord">
      <input v-model="note" placeholder="Antäckning">
      <input type="submit" value="Lägg till">
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
const HOST = import.meta.env.VITE_HOST;

const name = ref("")
const username = ref("")
const password = ref("")
const link = ref("")
const note = ref("")

const addItem = async () => {
  const res = {
    name: name.value,
    username: username.value,
    password: password.value,
    link: link.value,
    notes: note.value,
    club: "",
  }
  await fetch(HOST + "/api/" + "metadorerna" + "/add_supplier", {
    method: "POST",
    body: JSON.stringify(res),
  })
}
</script>

<style scoped></style>
