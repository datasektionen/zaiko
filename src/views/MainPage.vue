<template>
  <div class="main">
    <PanelTemplate title="Brister">
      <template #icon>
        <BellAlertIcon />
      </template>
      <template #content>
        <div class="shortageDiv">
          <ShortageTable :items="shortage" />
        </div>
      </template>
    </PanelTemplate>
    <PanelTemplate title="Statistik">
      <template #icon>
        <ChartBarIcon />
      </template>
      <template #content>
        <StatsPanel />
      </template>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue'
import ShortageTable from '@/components/ShortageTable.vue'
import StatsPanel from '@/components/StatsPanel.vue'
import { BellAlertIcon, ChartBarIcon } from '@heroicons/vue/24/outline'
import { ref } from 'vue'
import type { ItemGetResponse, StockGetResponse } from '@/types';
// import type { Notification } from '@/types';

// const HOST = import.meta.env.VITE_HOST;
const items = ref<Array<ItemGetResponse>>([]);
const shortage = ref<Array<StockGetResponse>>([]);


const GetData = () => {
  items.value = [ ];
  shortage.value = [
    {
      id: 1,
      name: 'Kaffe',
      location: 'Koket',
      current: 10,
      min: 5,
      order: 5
    },
    {
      id: 2,
      name: 'Mjolk',
      location: 'Kylskap',
      current: 2,
      min: 1,
      order: 3
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
    {
      id: 3,
      name: 'Socker',
      location: 'Skafferiet',
      current: 0,
      min: 1,
      order: 2
    },
  ];

}
GetData();
</script>

<style scoped>
.main {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  padding: 4rem;
  padding-bottom: 0;
}
</style>
