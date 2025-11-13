import type { LogQueryParams, Notification } from '@/types'
import { useNotificationsStore } from './notifications'

export async function getLog(item: LogQueryParams): Promise<any> {
  const query = new URLSearchParams(item)

  const res = await fetch('/api/log?' + query.toString(), { method: 'GET' })
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
  const log = await res.json()
  console.log('Fetched log:', log)
  return Promise.resolve(log)
}
