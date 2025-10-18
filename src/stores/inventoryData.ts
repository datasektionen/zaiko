import type { ShortageGetResponse, Notification } from "@/types";
import { useNotificationsStore } from "./notifications";


export async function getShortage(): Promise<ShortageGetResponse> {
  const res = await fetch('/api/shortage');
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
  const shortages: ShortageGetResponse = await res.json();
  console.log("Fetched shortages:", shortages);
  return Promise.resolve(shortages);
}

export function unitText(unit?: string | null): string {
  if(!unit) return "";
  if(unit == null) return "";
  return ` ${unit}`;
}

export function containerText(container?: string | null): string {
  if(!container) return "";
  if(container == null) return "";
  return `(${container})`;
}
