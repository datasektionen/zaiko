import type { StorageContainersGetResponse, Notification, StoragesGetResponse, StorageCreateRequest, StorageDeleteRequest, StorageEditRequest, StorageTreeGetResponse, StorageTreeRequest, StorageContainersTreeGetResponse } from "@/types";
import { useNotificationsStore } from "./notifications";

export async function getStorageContainers(): Promise<StorageContainersGetResponse> {
  const res = await fetch("/api/storages/containers", { method: "GET" });
  if (!res.ok) {
    const notificationsStore = useNotificationsStore();
    const noti: Notification = {
      id: Date.now(),
      title: res.statusText,
      message: await res.text(),
      severity: "error",
    }
    notificationsStore.add(noti);
    return Promise.reject(noti);
  }
  const storage = await res.json();
  console.log("Fetched storage:", storage);
  return Promise.resolve(storage);
}

export async function getStorageTree(query: StorageTreeRequest): Promise<StorageContainersTreeGetResponse> {
  const queryBody = new URLSearchParams(query);
  const res = await fetch("/api/storages/containers/items?" + queryBody.toString(), { method: "GET" });
  if (!res.ok) {
    const notificationsStore = useNotificationsStore();
    const noti: Notification = {
      id: Date.now(),
      title: res.statusText,
      message: await res.text(),
      severity: "error",
    }
    notificationsStore.add(noti);
    return Promise.reject(noti);
  }
  const storage = await res.json();
  console.log("Fetched storage tree:", storage);
  return Promise.resolve(storage);
}

export async function getStorages(): Promise<StoragesGetResponse> {
  const res = await fetch("/api/storages", { method: "GET" });
  if (!res.ok) {
    const notificationsStore = useNotificationsStore();
    const noti: Notification = {
      id: Date.now(),
      title: res.statusText,
      message: await res.text(),
      severity: "error",
    }
    notificationsStore.add(noti);
    return Promise.reject(noti);
  }
  const storages = await res.json();
  console.log("Fetched storages:", storages);
  return Promise.resolve(storages);
}

export async function createStorage(storage: StorageCreateRequest): Promise<void> {
  const res = await fetch("/api/storage", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(storage),
  });
  const notificationsStore = useNotificationsStore();
  if (!res.ok) {
    const noti: Notification = {
      id: Date.now(),
      title: res.statusText,
      message: await res.text(),
      severity: "error",
    }
    notificationsStore.add(noti);
    return Promise.reject(noti);
  }
  const noti: Notification = {
    id: Date.now(),
    title: "Lager skapat",
    message: `Lager "${storage.name}" är skapat.`,
    severity: "info",
  }
  notificationsStore.add(noti);
  console.log("Created storage:", storage);
  return Promise.resolve();
}

export async function deleteStorage(storage: StorageDeleteRequest): Promise<void> {
  const query = new URLSearchParams({ name: storage.name });
  const res = await fetch("/api/storage?" + query.toString(), { method: "DELETE" });
  const notificationsStore = useNotificationsStore();
  if (!res.ok) {
    const noti: Notification = {
      id: Date.now(),
      title: res.statusText,
      message: await res.text(),
      severity: "error",
    }
    notificationsStore.add(noti);
    return Promise.reject(noti);
  }
  const noti: Notification = {
    id: Date.now(),
    title: "Lager borttaget",
    message: `Lager "${storage.name}" är borttaget.`,
    severity: "info",
  }
  notificationsStore.add(noti);
  console.log("Deleted storage:", storage.name);
  return Promise.resolve();
}

export async function editStorage(body: StorageEditRequest): Promise<void> {
  const res = await fetch("/api/storage", {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(body),
  });
  const notificationsStore = useNotificationsStore();
  if (!res.ok) {
    const noti: Notification = {
      id: Date.now(),
      title: res.statusText,
      message: await res.text(),
      severity: "error",
    }
    notificationsStore.add(noti);
    return Promise.reject(noti);
  }
  const noti: Notification = {
    id: Date.now(),
    title: "Lager uppdaterat",
    message: `Lager "${body.name}" uppdaterades.`,
    severity: "info",
  }
  notificationsStore.add(noti);
  console.log("Edited storage:", body);
  return Promise.resolve();
}
