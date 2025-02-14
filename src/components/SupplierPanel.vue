<template>
  <div class="main-content">
    <h1>Redigera</h1>
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
      <div class="submit">
        <button @click="Delete()" class="delete">Ta bort</button>
        <input class="button" type="submit" value="Spara">
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { useClubsStore } from '@/stores/clubs';
import { useNotificationsStore } from '@/stores/notifications';
import type { SupplierGetResponse, SupplierUpdateRequest, Notification } from '@/types';
import { ref } from 'vue';
const HOST = import.meta.env.VITE_HOST;

const props = defineProps<{
  item: SupplierGetResponse
}>()

const emit = defineEmits(["deleted", "updated"]);

const notificationsStore = useNotificationsStore();
const clubStore = useClubsStore();

const name = ref(props.item.name)
const username = ref(props.item.username)
const password = ref(props.item.password)
const link = ref(props.item.link)
const note = ref(props.item.notes)
const id = ref(props.item.id)

const updateItem = async () => {
  const url: string = HOST + "/api/" + clubStore.getClub();
  const supplier: SupplierUpdateRequest = {
    id: id.value,
    name: name.value,
    username: username.value,
    password: password.value,
    link: link.value,
    notes: note.value,
  }
  await fetch(url + "/supplier", {
    method: "PATCH",
    body: JSON.stringify(supplier),
  })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Updaterad",
          message: "Leverantören har uppdaterats",
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
  emit("updated")
}

const Delete = async () => {
  const url: string = HOST + "/api/" + clubStore.getClub();

  await fetch(
    url + "/supplier?" + new URLSearchParams({ id: id.value.toString() }).toString(),
    {
      method: "DELETE",
    })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Borttagen",
          message: "Leverantören har tagits bort",
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
  emit("deleted")
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
  gap: 1.5rem;
  width: 100%;
}

fieldset {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2.25rem;
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
  max-width: 150px;
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
  justify-content: flex-end;
  gap: 2rem;
  flex-direction: row;
}

.delete {
  background-color: #eb4034;
  color: #fafafa;
}

@media (max-width: 700px) {

  .main-content {
    margin: 1rem 0;
    gap: 0.7rem;
  }

  fieldset {
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }

  h1 {
    font-size: 2.55rem;
  }
}
</style>
