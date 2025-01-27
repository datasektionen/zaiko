<template>
  <div class="main-content">
    <form v-on:submit.prevent="updateItem">
      <div class="item">
        <p>Namn</p>
        <input v-model="name" placeholder="Namn">
      </div>
      <div class="item">
        <p>Länk</p>
        <input v-model="link" placeholder="Länk">
      </div>
      <fieldset>
        <div class="item">
          <p>Användarnamn</p>
          <input v-model="username" placeholder="Användarnamn">
        </div>
        <div class="item">
          <p>Lösenord</p>
          <input v-model="password" placeholder="Lösenord">
        </div>
      </fieldset>
      <div class="item area">
        <p>Antäckningar</p>
        <textarea v-model="note" placeholder="Antäckningar"></textarea>
      </div>
      <input v-model="club" placeholder="Nämnd">
      <div class="submit">
        <input class="button" type="submit" value="Spara">
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import type { Supplier } from '@/types';
import { ref } from 'vue';
const HOST = import.meta.env.VITE_HOST;

const props = defineProps<{
  item: Supplier
}>()

const name = ref(props.item.name)
const username = ref(props.item.username)
const password = ref(props.item.password)
const link = ref(props.item.link)
const note = ref(props.item.notes)
const club = ref("metadorerna")

const updateItem = async () => {
  const res: Supplier = {
    name: name.value,
    username: username.value,
    password: password.value,
    link: link.value,
    notes: note.value,
    club: club.value,
  }
  await fetch(HOST + "/api/" + club.value + "/supplier", {
    method: "POST",
    body: JSON.stringify(res),
  })
}
</script>

<style scoped>
.main-content {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: flex-start;
  gap: 2rem;
  max-width: 1024px;
  margin: 2rem auto;
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
  width: 100%;
}

fieldset {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 3rem;
  width: 100%;
}

p {
  margin: 0;
}

.item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  padding: 8px 0;
}

.item input {
  border: none;
  background-color: inherit;
  text-align: right;
}

.area {
  flex-direction: column;
  align-items: flex-start;
}

.area p {
  padding-bottom: 8px;
  margin: 0;
  margin-bottom: 8px;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  width: auto;
}

.area textarea {
  border: none;
  background-color: inherit;
  min-height: 10rem;
  min-width: 100%;
}

.submit {
  display: flex;
  align-items: flex-end;
  flex-direction: column;
}
</style>
