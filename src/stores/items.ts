import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { ItemAddRequest, ItemGetResponse, ItemUpdateRequest, Notification } from "@/types";
import { useNotificationsStore } from '@/stores/notifications';
import { useClubsStore } from '@/stores/clubs';
import { useStockStore } from '@/stores/stock';

export const useItemStore = defineStore('items', () => {
  const HOST: string = import.meta.env.VITE_HOST;

  const notificationsStore = useNotificationsStore();
  const clubsStore = useClubsStore();
  const stockStore = useStockStore();

  const items = ref<Array<ItemGetResponse>>([]);

  async function fetchItems(): Promise<Array<ItemGetResponse>> {
    const club = await clubsStore.getClub();

    return fetch(HOST + "/api/" + club.name + "/item", {
      method: "GET",
    })
      .then((res) => res.json())
      .then((json: Array<ItemGetResponse>) => {
        items.value = json;
        return items.value;
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

  async function addItem(item: ItemAddRequest): Promise<ItemAddRequest> {
    const club = await clubsStore.getClub();

    if (club.permission !== "rw") {
      const noti: Notification = {
        id: Date.now(),
        title: "Icke behörig",
        message: "Du har inte behörighet att lägga till produkter",
        severity: "error",
      }
      notificationsStore.add(noti);
      return {} as ItemAddRequest;
    }

    return fetch(HOST + "/api/" + club.name + "/item", {
      method: "POST",
      body: JSON.stringify(item)
    })
      .then(() => fetchItems())
      .then(() => {
        const noti: Notification = {
          id: Date.now(),
          title: "Success",
          message: "Produkten lades till",
          severity: "info",
        }
        notificationsStore.add(noti);
        return item;
      })
      .then(async (item) => {
        await stockStore.fetchShortage();
        return item;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return {} as ItemAddRequest;
      });
  }

  async function updateItem(item: ItemUpdateRequest): Promise<ItemUpdateRequest> {
    const club = await clubsStore.getClub();

    if (club.permission !== "rw") {
      const noti: Notification = {
        id: Date.now(),
        title: "Icke behörig",
        message: "Du har inte behörighet att uppdatera produkter",
        severity: "error",
      }
      notificationsStore.add(noti);
      return {} as ItemUpdateRequest;
    }

    return fetch(HOST + "/api/" + club.name + "/item", {
      method: "PATCH",
      body: JSON.stringify(item)
    })
      .then(() => fetchItems())
      .then(() => {
        const noti: Notification = {
          id: Date.now(),
          title: "Success",
          message: "Produkten uppdaterades",
          severity: "info",
        }
        notificationsStore.add(noti);
        return item;
      })
      .then(async (item) => {
        await stockStore.fetchShortage();
        return item;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return {} as ItemUpdateRequest;
      });
  }

  async function deleteItem(id: number): Promise<ItemGetResponse> {
    const club = await clubsStore.getClub();

    if (club.permission !== "rw") {
      const noti: Notification = {
        id: Date.now(),
        title: "Icke behörig",
        message: "Du har inte behörighet att ta bort produkter",
        severity: "error",
      }
      notificationsStore.add(noti);
      return {} as ItemGetResponse;
    }

    const query = new URLSearchParams({ id: id.toString() }).toString();
    return fetch(HOST + "/api/" + club.name + "/item?" + query, {
      method: "DELETE"
    })
      .then(() => fetchItems())
      .then(() => {
        const noti: Notification = {
          id: Date.now(),
          title: "Success",
          message: "Produkten togs bort",
          severity: "info",
        }
        notificationsStore.add(noti);
        return {} as ItemGetResponse;
      })
      .then(async (item) => {
        await stockStore.fetchShortage();
        return item;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return {} as ItemGetResponse;
      });
  }

  async function getItem(id: number): Promise<ItemGetResponse> {
    if (items.value.length === 0) {
      await fetchItems();
    }
    const res = items.value.find((item) => item.id === id);
    if (res) {
      return res;
    }
    const noti: Notification = {
      id: Date.now(),
      title: "Error",
      message: "Kunde inte hämta produkt",
      severity: "error",
    }
    notificationsStore.add(noti);
    return {
      id: id,
      name: "",
      location: "",
      min: undefined,
      max: undefined,
      current: 0,
      supplier: undefined,
      link: undefined
    } as ItemGetResponse;
  }

  return { fetchItems, addItem, updateItem, deleteItem, getItem, items }
})
