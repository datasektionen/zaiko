<template>
  <div class="main">
    <PanelTemplate title="Inventera">
      <template #icon>
        <ClipboardDocumentListIcon />
      </template>
      <template #content>
        <Suspense>
          <StockTable :filter="filter" :key="key" />
        </Suspense>
      </template>
      <template #headerRight>
        <FilterPopup :columns="columns" @search="Filter" @clear="Clear()" />
      </template>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue'
import StockTable from '@/components/StockTable.vue'
import { ClipboardDocumentListIcon } from '@heroicons/vue/24/outline'
import FilterPopup from '@/components/FilterPopup.vue'
import { ArchiveBoxIcon, HomeIcon, ShoppingCartIcon, WalletIcon, InboxArrowDownIcon, ArrowsUpDownIcon } from '@heroicons/vue/16/solid'
import type { FilterColumn, FilterItemParams } from '@/types'
import { computed, ref } from 'vue'

const columns: Array<FilterColumn> = [
  { name: 'product', label: 'Produkt', icon: ArchiveBoxIcon },
  { name: 'location', label: 'Plats', icon: HomeIcon },
  { name: 'supplier', label: 'Leverantör', icon: ShoppingCartIcon },
  { name: 'quantity', label: 'Mängd', icon: WalletIcon },
  { name: 'new', label: 'Nya', icon: InboxArrowDownIcon },
  { name: 'difference', label: 'Differens', icon: ArrowsUpDownIcon },
];

const filter = ref<FilterItemParams | number>(0);

const Filter = (column: string, search: string) => {
  filter.value = { column, search }
}
const Clear = () => {
  filter.value = 0
}

const key = computed(() => {
  if (typeof filter.value === 'number') {
    return filter.value
  }
  return filter.value.column + filter.value.search
})

</script>

<style scoped>
.main {
  padding: 4rem;
  padding-bottom: 0;
}

@media (max-width: 768px) {
  .main {
    padding: 0.3rem;
  }
}
</style>
