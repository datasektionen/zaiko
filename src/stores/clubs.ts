import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { ClubGetRequest, ClubStorage, Notification } from "@/types";
import { useNotificationsStore } from './notifications';

export const useClubsStore = defineStore('clubs', () => {
  const HOST = import.meta.env.VITE_HOST;

  const notificationsStore = useNotificationsStore();
  const clubs = ref<ClubStorage>({ active: { name: "Nämnd", permission: "r" }, clubs: [] });

  async function fetchClubs(): Promise<ClubStorage> {
    // clubs.value = { active: { name: "metadorerna", permission: "rw" }, clubs: [{ name: "metadorerna", permission: "rw" }, { name: "sjukvård", permission: "r" }] };
    // return Promise.resolve(clubs.value);
    return await fetch(HOST + "/api/clubs", {
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
    await fetch(HOST + "/api/club?" + query.toString(), {
      method: "POST",
    })
      .then(() => fetchClubs())
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
