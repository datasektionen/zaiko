import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { Notification, SupplierAddRequest, SupplierGetResponse, SupplierListGetResponse, SupplierUpdateRequest } from "@/types";
import { useNotificationsStore } from '@/stores/notifications';

export const useSupplierStore = defineStore('supplier', () => {
  // const HOST: string = import.meta.env.VITE_HOST;

  const notificationsStore = useNotificationsStore();

  const suppliers = ref<Array<SupplierGetResponse>>([]);
  const supplierNames = ref<Map<number, string>>(new Map<number, string>());

  async function fetchSuppliers(): Promise<Array<SupplierGetResponse>> {
    return fetch("/api/admin/supplier", {
      method: "GET",
      credentials: "include",
    })
      .then((res) => res.json())
      .then((json: Array<SupplierGetResponse>) => {
        suppliers.value = json;
        return suppliers.value;
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

  async function addSupplier(supplier: SupplierAddRequest): Promise<SupplierAddRequest> {
    return fetch("/api/admin/supplier", {
      method: "POST",
      body: JSON.stringify(supplier),
      credentials: "include",
    })
      .then(() => fetchSuppliers())
      .then(() => {
        const noti: Notification = {
          id: Date.now(),
          title: "Success",
          message: "Leverantören lades till",
          severity: "info",
        }
        notificationsStore.add(noti);
        return supplier;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return {} as SupplierAddRequest;
      });
  }

  async function updateSupplier(supplier: SupplierUpdateRequest): Promise<SupplierUpdateRequest> {
    return fetch("/api/admin/supplier", {
      method: "PATCH",
      body: JSON.stringify(supplier),
      credentials: "include",
    })
      .then(() => fetchSuppliers())
      .then(() => {
        const noti: Notification = {
          id: Date.now(),
          title: "Success",
          message: "Leverantören uppdaterades",
          severity: "info",
        }
        notificationsStore.add(noti);
        return supplier;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return {} as SupplierUpdateRequest;
      });
  }

  async function deleteSupplier(id: number): Promise<SupplierGetResponse> {
    const query = new URLSearchParams({ id: id.toString() }).toString();
    return fetch("/api/admin/supplier?" + query, {
      method: "DELETE",
      credentials: "include",
    })
      .then(() => fetchSuppliers())
      .then(() => {
        const noti: Notification = {
          id: Date.now(),
          title: "Success",
          message: "Leverantörer togs bort",
          severity: "info",
        }
        notificationsStore.add(noti);
        return {} as SupplierGetResponse;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return {} as SupplierGetResponse;
      });
  }

  async function fetchSupplierNames(): Promise<Map<number, string>> {
    return fetch("/api/admin/suppliers", {
      method: "GET",
      credentials: "include",
    })
      .then((res) => res.json())
      .then((json: Array<SupplierListGetResponse>) => {
        json.forEach((supplier) => {
          supplierNames.value.set(supplier.id, supplier.name);
        });
        return supplierNames.value;
      })
      .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
        return new Map<number, string>();
      });
  }

  function getSupplierName(id?: number): string {
    if (id === undefined) {
      return "";
    }
    const name = supplierNames.value.get(id);
    if (name === undefined) {
      return "";
    }
    return name;
  }

  function getSupplierId(name?: string): number {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const id = Array.from(supplierNames.value.entries()).find(([_, value]) => value === name)?.[0];
    if (id === undefined) {
      return -1;
    }
    return id;
  }

  async function getSupplier(id: number): Promise<SupplierGetResponse> {
    if (suppliers.value.length === 0) {
      await fetchSuppliers();
    }
    const res = suppliers.value.find((supplier) => supplier.id === id);
    if (res) {
      return res;
    }
    const noti: Notification = {
      id: Date.now(),
      title: "Error",
      message: "Kunde inte hämta leverantör",
      severity: "error",
    }
    notificationsStore.add(noti);
    return {} as SupplierGetResponse;
  }

  return { fetchSuppliers, addSupplier, updateSupplier, deleteSupplier, fetchSupplierNames, getSupplierName, getSupplier, suppliers, supplierNames, getSupplierId }
})

