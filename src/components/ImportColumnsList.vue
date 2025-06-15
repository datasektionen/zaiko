<template>
  <div>
    <TitleMedium title="Kolumner">
      <ViewColumnsIcon />
    </TitleMedium>

    <div class="columnsList">
      <div v-for="rec in selectedTable" :key="rec.name" class="tableSelect">
        <div class="tableHeader">
          <component :is="rec.icon" />
          <h4>{{ rec.name }}</h4>
          <select v-model="selectedName[rec.name]" required>
            <option :value="" selected disabled>Tabell</option>
            <option v-for="rec in columns" :key="rec.name" :value="rec">{{ rec.label }}</option>
          </select>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import TitleMedium from './TitleMedium.vue';
import { ViewColumnsIcon } from '@heroicons/vue/24/outline';
import { NumberedListIcon } from '@heroicons/vue/20/solid';
import type { ImportColumn } from '@/types';

const props = defineProps<{
  items: Array<ImportColumn>,
  suppliers: Array<ImportColumn>,
  selected: "items" | "suppliers",
}>();

const selectedTable = computed((): Array<ImportColumn> => {
  return props.selected === "items" ? props.items : props.suppliers;
});

const selectedName = ref<Map<String, String>>(new Map());

</script>

<style scoped></style>
