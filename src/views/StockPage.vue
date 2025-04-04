<template>
  <div class="main">
    <PanelTemplate title="Inventera">
      <template #icon>
        <ClipboardDocumentListIcon />
      </template>
      <template #content>
        <Suspense>
          <StockTable />
          <template #fallback>
            <p>Laddar...</p>
          </template>
        </Suspense>
        <button @click="updateItems">
          <ClipboardDocumentListIconSolid />
          <p>Inventera</p>
        </button>
      </template>
      <template #headerRight>
        <FilterPopup :columns="columns" />
      </template>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue'
import StockTable from '@/components/StockTable.vue'
import { ClipboardDocumentListIcon } from '@heroicons/vue/24/outline'
import FilterPopup from '@/components/FilterPopup.vue'
import { ArchiveBoxIcon, HomeIcon, ShoppingCartIcon, WalletIcon, InboxArrowDownIcon, ArrowsUpDownIcon, ClipboardDocumentListIcon as ClipboardDocumentListIconSolid } from '@heroicons/vue/16/solid'
import type { FilterColumn } from '@/types'
import { useStockStore } from '@/stores/stock'

const columns: Array<FilterColumn> = [
  { name: 'product', label: 'Produkt', icon: ArchiveBoxIcon },
  { name: 'location', label: 'Plats', icon: HomeIcon },
  { name: 'supplier', label: 'Leverantör', icon: ShoppingCartIcon },
  { name: 'quantity', label: 'Mängd', icon: WalletIcon },
  { name: 'new', label: 'Nya', icon: InboxArrowDownIcon },
  { name: 'difference', label: 'Differens', icon: ArrowsUpDownIcon },
];

const stockStore = useStockStore();

const updateItems = async () => {
  stockStore.takeStock();
}

</script>

<style scoped>
.main {
  padding: 4rem;
  padding-bottom: 0;
}

button {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  border: none;
  border-radius: 0.5rem;
  background-color: #2EB563;
  color: #FAFAFA;
  cursor: pointer;
}

button p {
  margin: 0;
  font-size: 1.1rem;
}

button svg {
  width: 1.5rem;
  height: 1.5rem;
}

@media (max-width: 768px) {
  .main {
    padding: 0.3rem;
  }
}
</style>
