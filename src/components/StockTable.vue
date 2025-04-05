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
        <tr v-for="item, idx in itemStore.items" :key="item.id">
          <td scope="row">
            <a :href="item.link" target="_blank" v-if="item.link">{{ item.name }}</a>
            <p v-else>{{ item.name }}</p>
          </td>
          <td>{{ item.location }}</td>
          <td>{{ supplierStore.getSupplierName(item.supplier) }}</td>
          <td>{{ item.current }}</td>
          <td>
            <input v-model.number="stockStore.output.items[idx][1]" type="number">
          </td>
          <td :class="diffColor(stockStore.output.items[idx][1], item.current)">{{ diff(stockStore.output.items[idx][1],
            item.current) }}</td>
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
import { useStockStore } from '@/stores/stock';
import { useSupplierStore } from '@/stores/suppliers';
import { ArchiveBoxIcon, ShoppingCartIcon, HomeIcon, InboxArrowDownIcon, WalletIcon, ArrowsUpDownIcon } from '@heroicons/vue/16/solid'
import { useMediaQuery } from '@vueuse/core/index.cjs'
import EmptyTable from './EmptyTable.vue';
import { ArchiveBoxXMarkIcon } from '@heroicons/vue/24/outline';

const supplierStore = useSupplierStore();
const itemStore = useItemStore();
const stockStore = useStockStore();


const isMobile = useMediaQuery('(max-width: 768px)');

const diff = (newVal: number, current: number) => {
  return newVal - current
}

const diffColor = (newVal: number, current: number) => {
  return newVal - current < 0 ? 'red' : 'green'
}

if (stockStore.output.items.length === 0) {
  await stockStore.fetchStock();
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
  white-space: nowrap;
  overflow: hidden;
  max-width: 200px;
  border-left: 1px solid #DADADA;
  border-top: 1px solid #DADADA;
}

input {
  text-overflow: ellipsis;
  width: 100%;
  font-size: 1rem;
}

td:nth-child(5) {
  text-align: center;
  max-width: 33px;
  border: 3px solid #DADADA;
  background-color: #F5F5F5;
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
    max-width: 92%;
  }

  .icon {
    margin: 0 auto;
  }
}
</style>
