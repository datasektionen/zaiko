<template>
  <div>
    <table>
      <thead>
        <tr>
          <th scope="col">
            <span>
              <ArchiveBoxIcon class="icon" />
              <p v-if="!isMobile">Produkt</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <HomeIcon class="icon" />
              <p v-if="!isMobile">Plats</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <ShoppingCartIcon class="icon" />
              <p v-if="!isMobile">Leverantör</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <WalletIcon class="icon" />
              <p v-if="!isMobile">Mängd</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <InboxArrowDownIcon class="icon" />
              <p v-if="!isMobile">Nya</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <ArrowsUpDownIcon class="icon" />
              <p v-if="!isMobile">Differens</p>
            </span>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(item, idx) in items" :key="item.id">
          <td scope="row">
            <a :href="item.link" target="_blank" v-if="item.link">{{ item.name }}</a>
            <p v-else>{{ item.name }}</p>
          </td>
          <td>{{ item.location }}</td>
          <td>{{ item.supplier }}</td>
          <td>{{ item.current }}</td>
          <td>
            <input v-model.number="input.items[idx][1]" type="number">
          </td>
          <td :class="diffColor(input.items[idx][1], item.current)">{{ diff(input.items[idx][1], item.current) }}</td>
        </tr>
      </tbody>
    </table>
    <button @click="updateItems">
      <ClipboardDocumentListIcon />
      <p>Inventera</p>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { ItemGetResponse, StockUpdateRequest, Notification, FilterItemParams } from '@/types'
import { ArchiveBoxIcon, ShoppingCartIcon, HomeIcon, InboxArrowDownIcon, WalletIcon, ArrowsUpDownIcon, ClipboardDocumentListIcon } from '@heroicons/vue/16/solid'
import { useNotificationsStore } from '@/stores/notifications'
import { useClubsStore } from '@/stores/clubs'
import { useMediaQuery } from '@vueuse/core/index.cjs'

const { filter } = defineProps<{
  filter: FilterItemParams | number
}>()

const isMobile = useMediaQuery('(max-width: 768px)');

const diff = (newVal: number, current: number) => {
  return newVal - current
}

const diffColor = (newVal: number, current: number) => {
  return newVal - current < 0 ? 'red' : 'green'
}

const items = ref<Array<ItemGetResponse>>([]);
const input = ref<StockUpdateRequest>({ items: [] });
const HOST: string = import.meta.env.VITE_HOST;

const notificationsStore = useNotificationsStore();
const clubStore = useClubsStore();

const Filter = async () => {
  const url: string = HOST + "/api/" + clubStore.getClub() + "/item?";
  const params = new URLSearchParams(Object.entries(filter)).toString();
  fetch(url + params)
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
}

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
}
if (filter == 0) {
  GetData()
} else {
  Filter()
}

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
  if (filter == 0) {
    GetData()
  } else {
    Filter()
  }
}
</script>

<style scoped>
table {
  width: calc(100% - 2rem);
  border-collapse: collapse;
  margin: 3rem 1rem;
}

thead tr th:first-child,
td:first-child {
  border-left: none;
}

span {
  display: flex;
  align-items: center;
  justify-content: start;
}

th[scope="col"] {
  padding: 0.5rem;
  border-left: 1px solid #DADADA;
  color: #DADADA;
}

td {
  padding: 0.5rem;
  text-overflow: ellipsis;
  border-left: 1px solid #DADADA;
  border-top: 1px solid #DADADA;
}

input {
  width: 3rem;
  padding: 0.2rem;
  border: 1px solid #DADADA;
  border-radius: 0.5rem;
  text-overflow: ellipsis;
}

.red {
  color: #B62E3D;
}

.green {
  color: #2EB563;
}

.icon {
  margin-right: 0.5rem;
  width: 20px;
  height: 20px;
}

a {
  color: #2984BA;
  text-decoration: none;
}


button {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  border: none;
  border-radius: 0.5rem;
  background-color: #2EB563;
  color: #FAFAFA;
  cursor: pointer;
}

button p {
  margin: 0;
  font-size: 1.1rem;
}

button svg {
  width: 1.5rem;
  height: 1.5rem;
}

@media (max-width: 768px) {
  table {
    width: 100%;
    margin: 2rem 0;
    overflow-x: scroll;
  }

  td {
    white-space: nowrap;
    max-width: 100px;
  }
}
</style>
