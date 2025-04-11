import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { ItemAddRequest, ItemGetResponse, ItemUpdateRequest, Notification } from "@/types";
import { useNotificationsStore } from '@/stores/notifications';
import { useStockStore } from '@/stores/stock';

export const useItemStore = defineStore('items', () => {
  const notificationsStore = useNotificationsStore();
  const stockStore = useStockStore();

  const items = ref<Array<ItemGetResponse>>([]);

  async function fetchItems(): Promise<Array<ItemGetResponse>> {
    return fetch("/api/item", {
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
    return fetch("/api/admin/item", {
      method: "POST",
      body: JSON.stringify(item),
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
    return fetch("/api/admin/item", {
      method: "PATCH",
      body: JSON.stringify(item),
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
    const query = new URLSearchParams({ id: id.toString() }).toString();
    return fetch("/api/admin/item?" + query, {
      method: "DELETE",
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
