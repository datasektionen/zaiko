<template>
  <div>
    <div class="tableHeader">
      <div class="tableSelect">
        <h4>Lägg till i </h4>
        <component :is="filterType.icon" />
        <select v-model="filterType" required>
          <option :value="{ name: '', label: 'Tabell', icon: NumberedListIcon }" selected disabled>Tabell</option>
          <option v-for="rec in columns" :key="rec.name" :value="rec">{{ rec.label }}</option>
        </select>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { NumberedListIcon } from '@heroicons/vue/20/solid';
import type { FilterColumn } from '@/types';

const { columns } = defineProps<{
  columns: Array<FilterColumn>;
}>()

const filterType = ref<FilterColumn>({ name: '', label: 'Tabell', icon: NumberedListIcon });


</script>

<style scoped>

.tableHeader {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  font-size: 1.2rem;
}

.tableHeader svg {
  width: 21px;
  height: 21px;
  color: #DADADA;
}

.tableSelect svg {
  color: rgba(0, 0, 0, 0.33);
}

.tableSelect h4 {
  font-size: 1.2rem;
  margin-right: 0.5rem;
  font-weight: normal;
}

.tableSelect {
  display: flex;
  align-items: center;
  background-color: #DADADA;
  padding: 1rem 2rem;
  border-radius: 8px;
}

.tableSelect select {
  all: unset;
  appearance: auto;
  border-radius: 6px;
  margin-left: 0.3rem;
  min-width: 150px;
}
</style>
