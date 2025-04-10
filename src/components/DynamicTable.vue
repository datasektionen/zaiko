<template>
  <div>
    <table>
      <thead>
        <tr>
          <th v-for="column in columns" :key="column.label" scope="col">
            <span>
              <component :is="column.icon" class="icon" />
              <p v-if="!isMobile">{{ column.label }}</p>
            </span>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(row, rowIndex) in rows" :key="rowIndex">
          <td v-for="(column, colIndex) in columns" :key="colIndex">
            <a v-if="row.link && column.value == 'name'" :href="row.link" target="_blank">{{ row.name }}</a>
            <p v-else>{{ row[column.value] }}</p>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'
import type { TableColumn } from '@/types'
import { useMediaQuery } from '@vueuse/core';

const isMobile = useMediaQuery('(max-width: 768px)');

defineProps<{
  // eslint-disable-next-line  @typescript-eslint/no-explicit-any
  rows: Array<any>,
  columns: Array<TableColumn>,
}>()

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

a {
  color: #2984BA;
  text-decoration: none;
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
