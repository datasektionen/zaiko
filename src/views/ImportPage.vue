<template>
  <div class="main">
    <PanelTemplate title="Importera">
      <template #icon>
        <TableCellsIcon />
      </template>
      <template #content>
        <div style="width: 90%; height: 600px; background-color: #f1f1f1; border-radius: 8px;">

        </div>
      </template>
    </PanelTemplate>
    <PanelTemplate title="Konvertera">
      <template #icon>
        <CircleStackIcon />
      </template>
      <template #content>
        <div>
          <ImportMetadata :columns="tables" />
          <ImportColumnsList :columns="tables" />
        </div>
      </template>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue';
import ImportMetadata from '@/components/ImportMetadata.vue';
import { TableCellsIcon, CircleStackIcon } from '@heroicons/vue/24/outline';
import { ShoppingCartIcon, ArchiveBoxIcon } from '@heroicons/vue/16/solid';
import type { FilterColumn } from '@/types';
import ImportColumnsList from '@/components/ImportColumnsList.vue';

const tables: Array<FilterColumn> = [
  { name: 'product', label: 'Produkt', icon: ArchiveBoxIcon },
  { name: 'supplier', label: 'Leverantör', icon: ShoppingCartIcon },
];
</script>

<style scoped>
.main {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  padding: 4rem;
}

@media (max-width: 768px) {
  .main {
    grid-template-columns: 1fr;
    padding: 2rem;
  }
}
</style>
