<template>
  <div>
    <table v-if="stockStore.shortage.length > 0">
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
              <p v-if="!isMobile">Leverantör</p>
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
              <CurrencyDollarIcon class="icon" />
              <p v-if="!isMobile">Att kopa</p>
            </span>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in stockStore.shortage" :key="item.id">
          <td scope="row">{{ item.name }}</td>
          <td>{{ item.location }}</td>
          <td>{{ item.supplier }}</td>
          <td>{{ item.current }}</td>
          <td>{{ item.order }}</td>
        </tr>
      </tbody>
    </table>
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

const stockStore = useStockStore();

if (stockStore.shortage.length == 0) {
  await stockStore.fetchShortage();
}


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
