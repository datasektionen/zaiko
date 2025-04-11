import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { ItemGetResponse, Notification, StockGetResponse, StockUpdateRequest } from "@/types";
import { useNotificationsStore } from '@/stores/notifications';
import { useItemStore } from '@/stores/items';

export const useStockStore = defineStore('stock', () => {
  const notificationsStore = useNotificationsStore();
  const itemStore = useItemStore();

  const output = ref<StockUpdateRequest>({ items: [] });
  const shortage = ref<Array<StockGetResponse>>([]);

  async function fetchShortage(): Promise<Array<StockGetResponse>> {
    return fetch("/api/stock", {
      method: "GET",
      credentials: "include",
    })
      .then((res) => res.json())
      .then((json: Array<StockGetResponse>) => {
        shortage.value = json;
        return shortage.value;
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

  async function fetchStock(): Promise<Array<Array<number>>> {
    const items = await itemStore.fetchItems();

    output.value.items = items.map((item) => {
      return [
        item.id,
        item.current,
      ]
    });
    return output.value.items;
  }

  async function takeStock(): Promise<Array<ItemGetResponse>> {
    return fetch("/api/admin/stock", {
      method: "POST",
      body: JSON.stringify(output.value),
      credentials: "include",
    })
      .then(() => fetchShortage())
      .then(() => itemStore.fetchItems())
      .then((items) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Success",
          message: "Inventering lyckades",
          severity: "info",
        }
        notificationsStore.add(noti);
        return items;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return {} as Array<ItemGetResponse>;
      });
  }

  return { fetchShortage, takeStock, fetchStock, shortage, output }
})
