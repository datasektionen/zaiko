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
</style>
