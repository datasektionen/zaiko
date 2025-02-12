<template>
  <div class="main-content">
    <h1>Lägg till</h1>
    <form v-on:submit.prevent="addItem">
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
        <input class="button" type="submit" value="Lägg till">
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { useNotificationsStore } from '@/stores/notifications';
import type { SupplierAddRequest, Notification } from '@/types';
import { ref } from 'vue';
const HOST = import.meta.env.VITE_HOST;

const emit = defineEmits(["done"]);

const name = ref("")
const username = ref("")
const password = ref("")
const link = ref("")
const note = ref("")
const club = ref("metadorerna")

const notificationsStore = useNotificationsStore();

const addItem = async () => {
  const supplier: SupplierAddRequest = {
    name: name.value,
    username: username.value,
    password: password.value,
    link: link.value,
    notes: note.value,
  }
  await fetch(HOST + "/api/" + club.value + "/supplier", {
    method: "POST",
    body: JSON.stringify(supplier),
  })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Sparad",
          message: "Leverantören lades till",
          severity: "info",
        }
        notificationsStore.add(noti);
      } else {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: "Något gick fel",
          severity: "error",
        }
        notificationsStore.add(noti);
      }
    })
    .catch((error) => {
      const noti: Notification = {
        id: Date.now(),
        title: "Error",
        message: error.toString(),
        severity: "error",
      }
      notificationsStore.add(noti);
    })

  emit('done')
}
</script>

<style scoped>
.main-content {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: flex-start;
  gap: 2rem;
  max-width: 700px;
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
  width: 100%;
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

@media (max-width: 700px) {

  .main-content {
    margin: 0;
    gap: 1rem;
  }

  fieldset {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  h1 {
    font-size: 2.55rem;
  }
}
</style>
