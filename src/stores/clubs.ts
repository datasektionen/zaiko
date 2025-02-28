import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { ClubStorage, Notification } from "@/types";
import { useNotificationsStore } from './notifications';

export const useClubsStore = defineStore('clubs', () => {
  const HOST = import.meta.env.VITE_HOST;

  const notificationsStore = useNotificationsStore();
  const clubs = ref<ClubStorage>({ club: "", clubs: [], timestamp: 0 });

  const clubsStore = localStorage.getItem('clubs');
  if (clubsStore && Date.now() - JSON.parse(clubsStore).timestamp < 1000 * 60 * 60 * 24) {
    const parsed = JSON.parse(clubsStore);
    clubs.value = parsed;
  } else {
    fetch(HOST + "/api/clubs", {
      method: "GET",
    })
      .then((res) => res.json())
      .then((json) => clubs.value.clubs = json)
      .then(() => {
        if (clubs.value.clubs.length > 0) {
          clubs.value.timestamp = Date.now();
          clubs.value.club = clubs.value.clubs[0];
        } else {
          const noti: Notification = {
            id: Date.now(),
            title: "Nämnd",
            message: "Ingen nämnd hittades",
            severity: "error",
          }
          notificationsStore.add(noti);
          clubs.value.club = "Nämnd";
          clubs.value.clubs = ["Nämnd"];
          clubs.value.timestamp = 0;
        }
        localStorage.setItem('clubs', JSON.stringify(clubs.value));
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

  function setClub(club: string) {
    clubs.value.club = club;
    localStorage.setItem('clubs', JSON.stringify(clubs.value));
  }

  function getClub() {
    if (clubs.value.club === "") {
      return "Nämnd";
    }
    return clubs.value.club ?? "Nämnd";
  }

  function displayClub() {
    const club = getClub();
    const displayClub = club.split("-")[0];
    return displayClub;
  }

  return { clubs, setClub, getClub, displayClub }
})
