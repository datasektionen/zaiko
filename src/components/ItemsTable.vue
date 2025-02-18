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
              <Battery0Icon class="icon" />
              <p v-if="!isMobile">Min</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <Battery100Icon class="icon" />
              <p v-if="!isMobile">Max</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <InformationCircleIcon class="icon" />
              <p v-if="!isMobile">Status</p>
            </span>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item, idx in items" :key="item.id" @click="emit('select', idx)">
          <td scope="row">
            <a :href="item.link" target="_blank" @click.stop="" v-if="item.link">{{ item.name }}</a>
            <p v-else>{{ item.name }}</p>
          </td>
          <td>{{ item.location }}</td>
          <td>{{ supplier(item.supplier) }}</td>
          <td>{{ item.current }}</td>
          <td>{{ item.min }}</td>
          <td>{{ item.max }}</td>
          <td>{{ status(item.min, item.current) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { defineProps, ref } from 'vue'
import type { ItemGetResponse, SupplierListGetResponse, Notification } from '@/types'
import { ArchiveBoxIcon, ShoppingCartIcon, Battery0Icon, Battery100Icon, HomeIcon, WalletIcon, InformationCircleIcon } from '@heroicons/vue/16/solid'
import { useClubsStore } from '@/stores/clubs';
import { useNotificationsStore } from '@/stores/notifications';
import { useMediaQuery } from '@vueuse/core/index.cjs';
const HOST = import.meta.env.VITE_HOST;
const suppliers = ref<Array<SupplierListGetResponse>>([])

defineProps<{
  items: Array<ItemGetResponse>
}>()

const isMobile = useMediaQuery('(max-width: 768px)');

const status = (min?: number, current?: number) => {
  if (!min || !current) {
    return ' '
  } else if (current < min) {
    return '🛑'
  } else {
    return '✅'
  }
}

const supplier = (id?: number) => {
  return suppliers.value.find((s) => s.id === id)?.name ?? ' '
}

const clubStore = useClubsStore()
const notificationsStore = useNotificationsStore()
const emit = defineEmits(['select'])

const GetSuppliers = () => {
  const url: string = HOST + "/api/" + clubStore.getClub() + "/suppliers";
  fetch(url, {
    method: "GET",
  }).then((r) => r.json())
    .then((json) => {
      suppliers.value = json
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
GetSuppliers();
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

tr {
  cursor: pointer;
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
