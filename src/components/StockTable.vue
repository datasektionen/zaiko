<template>
  <div>
    <table v-if="itemStore.items.length > 0" class="table">
      <thead>
        <tr>
          <th scope="col">
            <span>
              <ArchiveBoxIcon class="tableIcon" />
              <p v-if="!isMobile">Produkt</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <HomeIcon class="tableIcon" />
              <p v-if="!isMobile">Plats</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <ShoppingCartIcon class="tableIcon" />
              <p v-if="!isMobile">Leverantör</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <WalletIcon class="tableIcon" />
              <p v-if="!isMobile">Mängd</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <InboxArrowDownIcon class="tableIcon" />
              <p v-if="!isMobile">Nya</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <ArrowsUpDownIcon class="tableIcon" />
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
          <td>{{ item.supplier }}</td>
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
import { ArchiveBoxIcon, ShoppingCartIcon, HomeIcon, InboxArrowDownIcon, WalletIcon, ArrowsUpDownIcon } from '@heroicons/vue/16/solid'
import { useMediaQuery } from '@vueuse/core'
import EmptyTable from './EmptyTable.vue';
import { ArchiveBoxXMarkIcon } from '@heroicons/vue/24/outline';

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
input {
  text-overflow: ellipsis;
  width: 100%;
  font-size: 1rem;
  color: var(--zaiko-text);
}

td:nth-child(5) {
  text-align: center;
  max-width: 33px;
  background-color: var(--zaiko-bg-2);
  border-bottom: 2px solid var(--zaiko-bg-0);
}

/* tr:nth-child(odd) td:nth-child(5) { */
/*   background-color: var(--zaiko-bg-1); */
/* } */

.red {
  color: var(--zaiko-bad-color);
}

.green {
  color: var(--zaiko-main-color);
}
</style>
