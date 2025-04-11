import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { ClubGetRequest, ClubStorage, Notification } from "@/types";
import { useNotificationsStore } from './notifications';
import { useItemStore } from './items';
import { useSupplierStore } from './suppliers';
import { useStockStore } from './stock';
import { useStatsStore } from './stats';
import router from '@/router';

export const useClubsStore = defineStore('clubs', () => {
  const notificationsStore = useNotificationsStore();
  const itemsStore = useItemStore();
  const suppliersStore = useSupplierStore();
  const stockStore = useStockStore();
  const statsStore = useStatsStore();
  const clubs = ref<ClubStorage>({ active: { name: "Nämnd", permission: "r" }, clubs: [] });

  async function fetchClubs(): Promise<ClubStorage> {
    return await fetch("/api/clubs", {
      method: "GET",
    })
      .then((res) => res.json())
      .then((json: ClubStorage) => {
        clubs.value = json;
        return clubs.value;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return {} as ClubStorage;
      });
  }

  async function setClub(club: ClubGetRequest) {
    const query = new URLSearchParams({ club: club.name });
    await fetch("/api/club?" + query.toString(), {
      method: "POST",
    })
      .then(() => fetchClubs())
      .then(() => itemsStore.fetchItems())
      .then(() => stockStore.fetchShortage())
      .then(() => statsStore.fetchStats())
      .then(() => {
        if (club.permission === "rw") {
          suppliersStore.fetchSuppliers();
        }
      })
      .then(() => {
        router.push('/')
      })
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

  async function getClub(): Promise<ClubStorage> {
    if (clubs.value) {
      await fetchClubs();
    }
    return clubs.value;
  }

  return { clubs, setClub, fetchClubs, getClub }
})
