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
          localStorage.setItem('clubs', JSON.stringify(clubs.value));
        }
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
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    window.methone_conf.update({
      login_text: displayClub(),
    })
  }

  function getClub() {
    return clubs.value.club;
  }

  function displayClub() {
    if (clubs.value.club.length > 10) {
      return clubs.value.club.substring(0, 9) + "...";
    }
    return clubs.value.club;
  }

  return { clubs, setClub, getClub, displayClub }
})
