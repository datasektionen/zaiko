<template>
  <div>
    <table v-if="itemStore.items.length > 0">
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
        <tr v-for="item in itemStore.items" :key="item.id" @click="emit('select', item.id)">
          <td scope="row">
            <a :href="item.link" target="_blank" @click.stop="" v-if="item.link">{{ item.name }}</a>
            <p v-else>{{ item.name }}</p>
          </td>
          <td>{{ item.location }}</td>
          <td>{{ item.supplier == undefined ? "-" : item.supplier }}</td>
          <td>{{ item.current }}</td>
          <td>{{ item.min }}</td>
          <td>{{ item.max }}</td>
          <td>{{ status(item.min, item.current) }}</td>
        </tr>
      </tbody>
    </table>
    <div v-else>
      <EmptyTable :compact="isMobile.value" text="Inga produkter" :icon="ArchiveBoxXMarkIcon" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useItemStore } from '@/stores/items';
import { useSupplierStore } from '@/stores/suppliers';
// import { useClubsStore } from '@/stores/clubs';
import { ArchiveBoxIcon, ShoppingCartIcon, Battery0Icon, Battery100Icon, HomeIcon, WalletIcon, InformationCircleIcon } from '@heroicons/vue/16/solid'
import { ArchiveBoxXMarkIcon } from '@heroicons/vue/24/outline';
import { useMediaQuery } from '@vueuse/core/index.cjs';
import EmptyTable from './EmptyTable.vue';
import { useClubsStore } from '@/stores/clubs';

const isMobile = useMediaQuery('(max-width: 768px)');

const status = (min?: number, current: number) => {
  if (!min) {
    return '✅'
  } else if (current < min) {
    return '🛑'
  } else {
    return '✅'
  }
}

const supplierStore = useSupplierStore();
const itemStore = useItemStore();
const clubStore = useClubsStore();
const club = (await clubStore.getClub()).active;

if (supplierStore.suppliers.length === 0 && club.permission == "rw") {
  await supplierStore.fetchSuppliers();
  await supplierStore.fetchSupplierNames();
}
if (itemStore.items.length === 0) {
  await itemStore.fetchItems();
}

const emit = defineEmits(['select'])

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
  cursor: default;
}

td {
  padding: 0.5rem;
  text-overflow: ellipsis;
  overflow: hidden;
  max-width: 200px;
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

td p,
a {
  max-width: 92%;
  overflow: hidden;
  text-overflow: ellipsis;
}

@media (max-width: 768px) {
  table {
    width: 100%;
    margin: 2rem 0;
    overflow-x: scroll;
  }

  td {
    max-width: 120px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .icon {
    margin: 0 auto;
  }
}

@media (max-width: 400px) {
  table {
    width: 100%;
    margin: 2rem 0;
    overflow-x: scroll;
  }

  td {
    text-overflow: ellipsis;
    overflow: hidden;
    max-width: 55px;
  }
}
</style>
