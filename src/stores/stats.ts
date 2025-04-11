import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { Notification, Stats } from "@/types";
import { useNotificationsStore } from '@/stores/notifications';

export const useStatsStore = defineStore('stats', () => {
  const notificationsStore = useNotificationsStore();

  const stats = ref<Stats>({ items: 0, shortages: 0, suppliers: 0 });

  async function fetchStats(): Promise<Stats> {
    return fetch("/api/stats", {
      method: "GET",
    })
      .then((res) => res.json())
      .then((json: Stats) => {
        stats.value = json;
        return stats.value;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return {} as Stats;
      });
  }

  return { fetchStats, stats }
})
