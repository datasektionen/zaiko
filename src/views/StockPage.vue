<template>
  <div class="main">
    <div class="panel">
      <div class="top-bar">
        <h2 class="brist">Inventera</h2>
        <div class="filter-bar">
          <p>Filter</p>
          <input type="text">
        </div>
      </div>
      <div class="header">
        <p>Produkt</p>
        <p>Leverantör</p>
        <p>Gammla</p>
        <p>Nya</p>
        <p>Differens</p>
      </div>
      <form v-on:submit.prevent="updateItems" class="items">
        <StockItem v-for="(item, idx) in items" :key="item.id" :item="item" v-model="input.items[idx][1]" />
        <div class="saveDiv">
          <input class="button" type="submit" value="Inventera" />
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import StockItem from '@/components/StockItem.vue';
import { useClubsStore } from '@/stores/clubs';
import { useNotificationsStore } from '@/stores/notifications';
import type { ItemGetResponse, StockUpdateRequest, Notification } from '@/types'
import { ref } from 'vue'
const items = ref<Array<ItemGetResponse>>([])
const input = ref<StockUpdateRequest>({ items: [] });
const HOST: string = import.meta.env.VITE_HOST;

const notificationsStore = useNotificationsStore();
const clubStore = useClubsStore();

const GetData = async () => {
  const url: string = HOST + "/api/" + clubStore.getClub();
  fetch(url + "/item")
    .then((res) => res.json())
    .then((json) => {
      items.value = json
      items.value.forEach((e: ItemGetResponse, idx) => input.value.items[idx] = [e.id, e.current])
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
  // console.log(items.value, input.value)
}
GetData();

const updateItems = async () => {
  const url: string = HOST + "/api/" + clubStore.getClub();
  await fetch(url + "/stock", {
    method: "POST",
    body: JSON.stringify(input.value),
  })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Sparad",
          message: "Inventeringen lyckades",
          severity: "info",
        }
        notificationsStore.add(noti);
      } else {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: "Inventeringen misslyckades",
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
  GetData()
}
</script>

<style scoped>
div {
  border-radius: 2px;
  background-color: #fafafa;
  box-shadow: none;
}

.main {
  margin: 3rem;
}

.saveDiv {
  margin-top: 1rem;
  display: flex;
  align-items: flex-end;
  flex-direction: column;
}

.panel {
  width: 480px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.header {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  padding: 0.5rem;
  text-align: center;
  font-weight: bold;
}

.items {
  display: flex;
  align-items: stretch;
  flex-direction: column;
}

.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.filter-bar {
  background-color: rgba(224, 242, 241, 1);
  display: flex;
  gap: 1rem;
  padding: 10px 20px;
  margin: 0;
  align-items: center;
  justify-content: space-between;
}

.filter-bar p {
  margin: 0;
  text-align: center;
}

.brist {
  padding-bottom: 1rem;
  margin: 0;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
}

@media (max-width: 1024px) {
  .panel {
    grid-template-columns: 100%;
    width: 100%;
  }

  .main {
    margin: 1rem;
  }

  button {
    padding: 0.5rem 1rem;
  }

  .top-bar {
    padding: 0.7rem 0.8rem;
    gap: 1rem;
  }

  .filter-bar {
    padding: 0.7rem 0.8rem;
  }

  input {
    max-width: 150px;
  }
}
</style>
