<template>
  <div>
    <table class="table">
      <thead>
        <tr>
          <th v-for="column in columns" :key="column.label" scope="col">
            <span>
              <component :is="column.icon" class="tableIcon" />
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
.table {
  width: 100%;
  max-width: 100vw;
  border-collapse: collapse;
  margin: 0 auto;
  background-color: var(--zaiko-bg-0);
  padding: 2rem;
  border-radius: 8px;
  color: var(--zaiko-text);
}

thead tr th:first-child,
td:first-child {
  border-left: none;
}

.table span {
  display: flex;
  align-items: center;
  justify-content: start;
}

.table a {
  color: var(--zaiko-link-color);
  text-decoration: none;
}

.table th[scope="col"] {
  padding: 0.5rem;
  color: var(--zaiko-bg-2);
  border-bottom: 2px solid var(--zaiko-bg-1);
}

.table td {
  padding: 0.5rem;
  text-overflow: ellipsis;
  border-top: 1px solid var(--zaiko-border-color);
}

.tableIcon {
  margin-right: 0.5rem;
  width: 20px;
  height: 20px;
}

@media (max-width: 1200px) {
  .table {
    width: 100%;
    margin: 2rem 0;
    overflow-x: scroll;
  }

  .table td {
    text-overflow: ellipsis;
    overflow: hidden;
    max-width: 100px;
  }

}

@media (max-width: 768px) {
  .tableIcon {
    margin: 0 auto;
  }
}

@media (max-width: 400px) {
  .table td {
    text-overflow: ellipsis;
    overflow: hidden;
    max-width: 80px;
  }
}
</style>
