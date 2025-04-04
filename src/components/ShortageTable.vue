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
        <p v-if="stockStore.shortage.length == 0">Inga brister</p>
        <tr v-for="item in stockStore.shortage" :key="item.id" v-else>
          <td scope="row">{{ item.name }}</td>
          <td>{{ item.location }}</td>
          <td>{{ item.current }}</td>
          <td>{{ item.order }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ArchiveBoxIcon, HomeIcon, WalletIcon, CurrencyDollarIcon } from '@heroicons/vue/16/solid'
import { useMediaQuery } from '@vueuse/core/index.cjs';
import { useStockStore } from '@/stores/stock';

const stockStore = useStockStore();

if (stockStore.shortage.length == 0) {
  await stockStore.fetchShortage();
}


const isMobile = useMediaQuery('(max-width: 768px)');

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
  }
}
</style>
