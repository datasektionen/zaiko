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
              <Battery0Icon class="icon" />
              <p>Min</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <Battery100Icon class="icon" />
              <p>Max</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <InformationCircleIcon class="icon" />
              <p>Status</p>
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
          <td>{{ item.supplier }}</td>
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
import { defineProps } from 'vue'
import type { ItemGetResponse } from '@/types'
import { ArchiveBoxIcon, ShoppingCartIcon, Battery0Icon, Battery100Icon, HomeIcon, WalletIcon, InformationCircleIcon } from '@heroicons/vue/16/solid'

defineProps<{
  items: Array<ItemGetResponse>
}>()

const status = (min?: number, current?: number) => {
  if (!min || !current) {
    return ' '
  } else if (current < min) {
    return '🛑'
  } else {
    return '✅'
  }
}

const emit = defineEmits(['select'])

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
</style>
