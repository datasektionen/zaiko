<template>
  <div>
    <DynamicTable v-if="stockStore.shortage.length > 0" :columns="columns" :rows="stockStore.shortage" />
    <div v-else>
      <EmptyTable :compact="isMobile.value" text="Inga brister" :icon="HandThumbUpIcon" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ArchiveBoxIcon, HomeIcon, ShoppingCartIcon, WalletIcon, CurrencyDollarIcon } from '@heroicons/vue/16/solid'
import { HandThumbUpIcon } from '@heroicons/vue/24/outline'
import { useMediaQuery } from '@vueuse/core/index.cjs';
import { useStockStore } from '@/stores/stock';
import EmptyTable from '@/components/EmptyTable.vue';
import DynamicTable from '@/components/DynamicTable.vue';
import type { TableColumn } from '@/types';

const stockStore = useStockStore();

if (stockStore.shortage.length == 0) {
  await stockStore.fetchShortage();
}

const columns: Array<TableColumn> = [
  { label: 'Produkt', icon: ArchiveBoxIcon, value: 'name' },
  { label: 'Plats', icon: HomeIcon, value: 'location' },
  { label: 'Leverantör', icon: ShoppingCartIcon, value: 'supplier' },
  { label: 'Mängd', icon: WalletIcon, value: 'current' },
  { label: 'Att köpa', icon: CurrencyDollarIcon, value: 'order' }
];


const isMobile = useMediaQuery('(max-width: 768px)');

</script>

<style scoped>
table {
  width: 95%;
  max-width: 100vw;
  border-collapse: collapse;
  margin: 2.5rem 1rem;
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

.icon {
  margin-right: 0.5rem;
  width: 20px;
  height: 20px;
}

@media (max-width: 1200px) {
  table {
    width: 100%;
    margin: 2rem 0;
    overflow-x: scroll;
  }

  td {
    text-overflow: ellipsis;
    overflow: hidden;
    max-width: 100px;
  }

}

@media (max-width: 768px) {
  .icon {
    margin: 0 auto;
  }
}

@media (max-width: 400px) {
  td {
    text-overflow: ellipsis;
    overflow: hidden;
    max-width: 80px;
  }
}
</style>
