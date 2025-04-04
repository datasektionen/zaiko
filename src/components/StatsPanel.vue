<template>
  <div class="statsContainer">
    <div class="boxStats">
      <TitleMedium title="Produkter">
        <ArchiveBoxIcon />
      </TitleMedium>
      <TitleMedium title="Leverantörer" class="skip">
        <ShoppingCartIcon />
      </TitleMedium>
      <BoxData class="box" title="Totalt" :amount="statsStore.stats.items" good />
      <BoxData class="box" title="Att köpa" :amount="statsStore.stats.shortages" />
      <BoxData class="box" title="Totalt" :amount="statsStore.stats.suppliers" good />
    </div>
    <div class="graphStats">
      <TitleMedium title="Total Mängd">
        <ArchiveBoxIcon />
      </TitleMedium>
      <div class="graphDiv">
        <h6>WIP</h6>
      </div>
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
.graphDiv {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 220px;
  width: 100%;
  background-color: rgba(0, 0, 0, 0.3);
  border-radius: 6px;
}

.graphDiv h6 {
  color: white;
  font-size: 4rem;
}

.statsContainer {
  display: flex;
  flex-direction: column;
  gap: 3rem;
}

.boxStats {
  display: grid;
  align-items: center;
  grid-template-columns: repeat(3, 1fr);
  margin-bottom: 20px;
}

.box {
  margin-right: 20px;
  width: 150px;
  margin: auto;
}

.skip {
  grid-column: 3;
}

@media (max-width: 768px) {
  .boxStats {
    grid-template-columns: 1fr 1fr;
    max-width: 100%;
    gap: 20px;
  }

  .skip {
    grid-column: 2;
  }

  .box {
    margin-right: 0;
    margin: 0;
  }

  .box:nth-last-child(2) {
    grid-column: 1;
  }

  .box:last-child {
    grid-column: 2;
    grid-row: 2;
  }
}
</style>
