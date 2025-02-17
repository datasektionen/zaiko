<template>
  <div class="statsContainer">
    <div class="boxStats">
      <TitleMedium title="Produkter">
        <ArchiveBoxIcon />
      </TitleMedium>
      <TitleMedium title="Leverantörer" class="skip">
        <ShoppingCartIcon />
      </TitleMedium>
      <BoxData class="box" title="Totalt" :amount="total" good />
      <BoxData class="box" title="Att köpa" :amount="toBuy" />
      <BoxData class="box" title="Totalt" :amount="totalSuppliers" good />
    </div>
    <div class="graphStats">
      <TitleMedium title="Total Mängd">
        <ArchiveBoxIcon />
      </TitleMedium>
    </div>
  </div>
</template>

<script setup lang="ts">
import TitleMedium from '@/components/TitleMedium.vue'
import BoxData from '@/components/BoxData.vue'
import { ArchiveBoxIcon, ShoppingCartIcon } from '@heroicons/vue/24/outline'
import { ref } from 'vue';

const total = ref<number>(100);
const toBuy = ref<number>(5);
const totalSuppliers = ref<number>(8);

</script>

<style scoped>

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
  .box:nth-last-child(2) {
    grid-column: 1;
  }
  .box:last-child {
    grid-column: 2;
    grid-row: 2;
  }
}
</style>
