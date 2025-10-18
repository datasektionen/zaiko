<template>
  <div class="grid grid-cols-1 gap-2 md:grid-cols-[12fr_4fr] m-2 md:m-4">
    <PanelTemplate title="Lagerbrist" :icon="BellAlertIcon">
      <DynamicTable :rows="rows" :columns="columns">
        <template #row="input">
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <RouterLink :to="'/item/' + input.row.name">
              <p class="hover:underline">{{ input.row.name }}</p>
            </RouterLink>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <RouterLink :to="storagePath(input.row.storage, input.row.container)">
            <p class="hover:underline">{{ input.row.storage }}{{containerText(input.row.container)}}</p>
            </RouterLink>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <p>{{ input.row.amount + unitText(input.row.unit) }}</p>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <p>{{ input.row.amount_to_buy + unitText(input.row.unit)  }}</p>
          </td>
        </template>
      </DynamicTable>
    </PanelTemplate>
    <PanelTemplate title="Statistik" :icon="ChartBarIcon">
      <div class="flex gap-4">
        <BoxData title="Totalt" :amount="stats.items" good />
        <BoxData title="Att köpa" :amount="stats.shortages" />
      </div>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue'
import BoxData from '@/components/BoxData.vue'
import DynamicTable from '@/components/DynamicTable.vue'
import { BellAlertIcon, ChartBarIcon } from '@heroicons/vue/24/outline'
import type { ShortageGetResponse, StatsGetResponse } from '@/types';
import { ref } from 'vue';
import { getStats } from '@/stores/statsData';
import { getShortage, unitText, containerText } from '@/stores/inventoryData'

function storagePath(storage: string, container?: string) {
  return '/storage/' + storage + (container ? '?container=' + container : '');
}

const columns = {
  name: 'Namn',
  storage: 'Lager',
  amount: 'Mängd',
  amount_to_buy: 'Att köpa',
};

const rows = ref<ShortageGetResponse>([]);
getShortage().then((data) => {
  rows.value = data;
});

const stats = ref<StatsGetResponse>({ items: 0, shortages: 0, suppliers: 0 });
getStats().then((data) => {
  stats.value = data;
});

</script>

<style scoped></style>
