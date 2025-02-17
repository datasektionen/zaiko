<template>
  <div>
    <table>
      <thead>
        <tr>
          <th scope="col">
            <span>
              <ShoppingCartIcon class="icon" />
              <p>Namn</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <UserCircleIcon class="icon" />
              <p>Användarnamn</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <LockClosedIcon class="icon" />
              <p>Lösenord</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <DocumentTextIcon class="icon" />
              <p>Mangd</p>
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
          <td>{{ item.username }}</td>
          <td>{{ item.password }}</td>
          <td>{{ item.notes }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'
import type { SupplierGetResponse } from '@/types'
import { LockClosedIcon, ShoppingCartIcon, UserCircleIcon, DocumentTextIcon } from '@heroicons/vue/16/solid'

defineProps<{
  items: Array<SupplierGetResponse>
}>()

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
