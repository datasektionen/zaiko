import { ref } from 'vue'
import { defineStore } from 'pinia'
import { useNotificationsStore } from './notifications';
import type { UserInfoGetResponse, Notification } from '@/types';
import { fetchOrRedirect } from '@/common';

export const usePermsStore = defineStore('permissions', () => {
  const perms = ref<UserInfoGetResponse>()

  async function fetchPermissions(): Promise<UserInfoGetResponse | undefined> {
    const res = await fetchOrRedirect("/api/userinfo", { method: "GET" });
    const notificationsStore = useNotificationsStore();
    if (!res.ok) {
      const noti: Notification = {
        id: Date.now(),
        title: res.statusText,
        message: await res.text(),
        severity: "error",
      };
      notificationsStore.add(noti);
      return Promise.reject(noti);
    }
    perms.value = await res.json();
    console.log("Fetched permissions:", perms.value);
    return Promise.resolve(perms.value);
  }

  function isAdmin(): boolean {
    return perms.value?.permissions.find(p => p.id === 'admin') !== undefined;
  }

  function hasWriteAccess(): boolean {
    return perms.value?.permissions.find(p => p.id === 'write' || p.id === 'admin') !== undefined;
  }

  function writeAccessToStorage(storageName: string): boolean {
    if (isAdmin()) {
      return true;
    }
    return perms.value?.permissions.find(sp => sp.id === 'write' && sp.scope === storageName.toLowerCase()) !== undefined;
  }

  function hasGroup(): boolean {
    return perms.value?.groups.length! > 0;
  }

  return { perms, isAdmin, hasWriteAccess, writeAccessToStorage, hasGroup, fetchPermissions }
})
