<template>
  <div class="main">
    <PanelTemplate title="Brister">
      <template #icon>
        <BellAlertIcon />
      </template>
      <template #content>
        <div class="shortageDiv">
          <Suspense>
            <ShortageTable />
            <template #fallback>
              <p>Laddar...</p>
            </template>
          </Suspense>
        </div>
      </template>
    </PanelTemplate>
    <PanelTemplate title="Statistik">
      <template #icon>
        <ChartBarIcon />
      </template>
      <template #content>
        <Suspense>
          <StatsPanel />
          <template #fallback>
            <p>Laddar...</p>
          </template>
        </Suspense>
      </template>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue'
import ShortageTable from '@/components/ShortageTable.vue'
import StatsPanel from '@/components/StatsPanel.vue'
import { BellAlertIcon, ChartBarIcon } from '@heroicons/vue/24/outline'

</script>

<style scoped>
.main {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  padding: 4rem;
  padding-bottom: 0;
}

@media (max-width: 1150px) {
  .main {
    padding: 4rem 2rem;
  }
}

@media (max-width: 1000px) {
  .main {
    grid-template-columns: 1fr;
    padding: 0.4rem;
  }
}
</style>
