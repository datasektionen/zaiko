<template>
  <div class="main">
    <PanelTemplate title="Inventera">
      <template #icon>
        <ClipboardDocumentListIcon />
      </template>
      <template #content>
        <Suspense>
          <StockTable />
        </Suspense>
      </template>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue'
import StockTable from '@/components/StockTable.vue'
import { ClipboardDocumentListIcon } from '@heroicons/vue/24/outline'
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
