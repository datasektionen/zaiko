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
import type { StockGetResponse } from '@/types';
import type { Notification } from '@/types';
import { useNotificationsStore } from '@/stores/notifications'
import { useClubsStore } from '@/stores/clubs';

const HOST = import.meta.env.VITE_HOST;
const shortage = ref<Array<StockGetResponse>>([]);

const notificationsStore = useNotificationsStore();
const clubStore = useClubsStore();

const GetData = () => {
  if (!clubStore.checkClub()) return;
  const url: string = HOST + "/api/" + clubStore.getClub().name;

  fetch(url + "/stock")
    .then((res) => res.json())
    .then((json: Array<StockGetResponse>) => shortage.value = json)
    .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
    })

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
