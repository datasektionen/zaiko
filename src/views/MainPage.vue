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
              <SpinnerSimple color="#DADADA" />
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
            <SpinnerSimple color="#DADADA" />
          </template>
        </Suspense>
      </template>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue'
import ShortageTable from '@/components/ShortageTable.vue'
import SpinnerSimple from '@/components/SpinnerSimple.vue'
import StatsPanel from '@/components/StatsPanel.vue'
import { BellAlertIcon, ChartBarIcon } from '@heroicons/vue/24/outline'

</script>

<style scoped>
.main {
  display: grid;
  grid-template-columns: 12fr 4fr;
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
