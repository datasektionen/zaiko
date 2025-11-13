import type { StatsGetResponse, Notification } from '@/types'
import { useNotificationsStore } from './notifications'

export async function getStats(): Promise<StatsGetResponse> {
  const res = await fetch('/api/stats', { method: 'GET' })
  if (!res.ok) {
    const notificationsStore = useNotificationsStore()
    const noti: Notification = {
      id: Date.now(),
      title: res.statusText,
      message: await res.text(),
      severity: 'error',
    }
    notificationsStore.add(noti)
    return Promise.reject(noti)
  }
  const stats = await res.json()
  console.log('Fetched stats:', stats)
  return Promise.resolve(stats)
}
