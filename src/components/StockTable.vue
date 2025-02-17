<template>
  <div>
    <table>
      <thead>
        <tr>
          <th scope="col">
            <span>
              <ArchiveBoxIcon class="icon" />
              <p>Produkt</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <HomeIcon class="icon" />
              <p>Plats</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <ShoppingCartIcon class="icon" />
              <p>Leverantor</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <WalletIcon class="icon" />
              <p>Mangd</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <InboxArrowDownIcon class="icon" />
              <p>Nya</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <ArrowsUpDownIcon class="icon" />
              <p>Differens</p>
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
import type { ItemGetResponse, StockUpdateRequest } from '@/types'
import { ArchiveBoxIcon, ShoppingCartIcon, HomeIcon, InboxArrowDownIcon, WalletIcon, ArrowsUpDownIcon, ClipboardDocumentListIcon } from '@heroicons/vue/16/solid'

const diff = (newVal: number, current: number) => {
  return newVal - current
}

const diffColor = (newVal: number, current: number) => {
  return newVal - current < 0 ? 'red' : 'green'
}

const items = ref<Array<ItemGetResponse>>([]);
const input = ref<StockUpdateRequest>({ items: [] });

const GetData = async () => {
  items.value = [
    {
      id: 1,
      current: 10,
      name: "Kaffe",
      location: "Kök",
      updated: 5,
      link: "https://www.google.com",
      max: 20,
      min: 5,
      supplier: 0,
    },
    {
      id: 2,
      current: 5,
      name: "Te",
      location: "Kök",
      updated: 5,
      link: "https://www.google.com",
      max: 20,
      min: 5,
      supplier: 0,
    },
    {
      id: 3,
      current: 15,
      name: "Kakor",
      location: "Kök",
      updated: 5,
      link: "https://www.google.com",
      max: 20,
      min: 5,
      supplier: 0,
    },
  ];
  input.value.items = items.value.map((item) => [item.id, item.current]);
  console.log(items.value, input.value)
}
await GetData();

const updateItems = () => {
  console.log(input.value)
  // GetData()
}

</script>

<style scoped>
table {
  width: 85%;
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
</style>
