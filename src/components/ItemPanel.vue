<template>
  <div class="main-content">
    <h1>Redigera</h1>
    <form v-on:submit.prevent="updateItem">
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
        <input type="number" v-model="supplier" placeholder="Leverantör">
      </div>
      <div class="item">
        <p>Länk</p>
        <input v-model="link" placeholder="Länk">
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
import type { ItemGetResponse, ItemUpdateRequest, Notification } from '@/types';
import { ref } from 'vue'
const HOST = import.meta.env.VITE_HOST;

const props = defineProps<{
  item: ItemGetResponse
}>()

const emit = defineEmits(["deleted", "updated"]);

const notificationsStore = useNotificationsStore();

const name = ref(props.item.name)
const location = ref(props.item.location)
const min = ref(props.item.min)
const max = ref(props.item.max)
const current = ref(props.item.current)
const supplier = ref(props.item.supplier)
const link = ref(props.item.link)

const clubStore = useClubsStore();


const updateItem = async () => {
  const res: ItemUpdateRequest = {
    id: props.item.id,
    name: name.value,
    location: location.value,
    min: min.value,
    max: max.value,
    current: current.value,
    supplier: supplier.value,
    link: link.value,
  }
  const url: string = HOST + "/api/" + clubStore.getClub();
  await fetch(url + "/item", {
    method: "PATCH",
    body: JSON.stringify(res),
  })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Updaterad",
          message: "Produkten har uppdaterats",
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
  await fetch(url + "/item", {
    method: "DELETE",
    body: JSON.stringify({ name: name.value })
  })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Borttagen",
          message: "Produkten har tagits bort",
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
  gap: 1.75rem;
  width: 100%;
}

fieldset {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 2rem;
  width: 100%;
}

fieldset .item input {
  max-width: 100px;
}

.item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  padding: 8px 0;
  margin-bottom: 8px;
  max-width: 100%;
}

.item input {
  border: none;
  background-color: inherit;
  text-align: right;
  max-width: 150px;
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
    margin: 0;
    gap: 1rem;
  }

  fieldset {
    grid-template-columns: 1fr;
    gap: 0.66rem;
  }

  h1 {
    font-size: 2.55rem;
  }
}
</style>
