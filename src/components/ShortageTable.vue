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
              <WalletIcon class="icon" />
              <p>Mangd</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <CurrencyDollarIcon class="icon" />
              <p>Att kopa</p>
            </span>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in items" :key="item.id">
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
import { defineProps } from 'vue'
import type { StockGetResponse } from '@/types'
import { ArchiveBoxIcon, HomeIcon, WalletIcon, CurrencyDollarIcon } from '@heroicons/vue/16/solid'

defineProps<{
  items: Array<StockGetResponse>
}>()

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
</style>
