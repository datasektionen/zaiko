import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { ClubGetRequest, ClubStorage, Notification } from "@/types";
import { useNotificationsStore } from './notifications';

export const useClubsStore = defineStore('clubs', () => {
  const HOST = import.meta.env.VITE_HOST;

  const notificationsStore = useNotificationsStore();
  const clubs = ref<ClubStorage>({ club: { name: "Nämnd", permission: "r" }, clubs: [], timestamp: 0 });

  const clubsStore = localStorage.getItem('clubs');
  if (clubsStore && Date.now() - JSON.parse(clubsStore).timestamp < 1000 * 60 * 60) {
    const parsed = JSON.parse(clubsStore);
    clubs.value = parsed;
  } else {
    fetchClubs();
  }

  async function fetchClubs(): Promise<Array<ClubGetRequest>> {
    console.log("Fetching clubs");
    return await fetch(HOST + "/api/clubs", {
      method: "GET",
    })
      .then((res) => res.json())
      .then((json: Array<ClubGetRequest>) => {
        clubs.value.clubs = json;
        clubs.value.timestamp = Date.now();
        clubs.value.club = clubs.value.clubs[0];
        localStorage.setItem('clubs', JSON.stringify(clubs.value));
        return clubs.value.clubs;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return [];
      });
  }

  async function setClub(club: ClubGetRequest) {
    if (clubs.value.club.name === "Nämnd") {
      await fetchClubs();
    }
    clubs.value.club = club;
    localStorage.setItem('clubs', JSON.stringify(clubs.value));
    console.log(clubs.value);
  }

  async function getClub() {
    if (clubs.value.club.name === "Nämnd") {
      await fetchClubs();
    }
    return clubs.value.club;
  }

  return { clubs, setClub, getClub, fetchClubs }
})
