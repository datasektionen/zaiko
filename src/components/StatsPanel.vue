<template>
  <div class="boxStats">
    <TitleMedium title="Produkter">
      <ArchiveBoxIcon />
    </TitleMedium>
    <div class="boxDiv">
      <BoxData class="box" title="Totalt" :amount="statsStore.stats.items" good />
      <BoxData class="box" title="Att köpa" :amount="statsStore.stats.shortages" />
    </div>
    <TitleMedium title="Leverantörer">
      <ShoppingCartIcon />
    </TitleMedium>
    <div class="boxDiv">
      <BoxData class="box" title="Totalt" :amount="statsStore.stats.suppliers" good />
    </div>
  </div>
</template>

<script setup lang="ts">
import TitleMedium from '@/components/TitleMedium.vue'
import BoxData from '@/components/BoxData.vue'
import { ArchiveBoxIcon, ShoppingCartIcon } from '@heroicons/vue/24/outline'
import { useStatsStore } from '@/stores/stats'

const statsStore = useStatsStore();
await statsStore.fetchStats();

</script>

<style scoped>
.boxStats {
  margin-bottom: 20px;
  display: flex;
  flex-direction: column;
  gap: 20px;
  margin: 1rem;
}

.boxDiv {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  gap: 20px;
}

.box {
  width: 150px;
}

@media (max-width: 1000px) {
  .boxStats {
    max-width: 100%;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .boxStats {
    display: grid;
    grid-template-columns: repeat(1, 1fr);
  }
}
</style>
