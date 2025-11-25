import type {
  Notification,
  StockTreeGetResponse,
  StockUpdateRequest,
} from '@/types'
import { useNotificationsStore } from './notifications'

export async function getStockTree(): Promise<StockTreeGetResponse> {
  const res = await fetch('/api/inventory', { method: 'GET' })
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
  const tree = await res.json()
  console.log('Fetched stock tree:', tree)
  return Promise.resolve(tree)
}

export async function takeStock(body: StockUpdateRequest): Promise<void> {
  const res = await fetch('/api/inventory', {
    method: 'POST',
    body: JSON.stringify(body),
    headers: {
      'Content-Type': 'application/json',
    },
  })
  const notificationsStore = useNotificationsStore()
  if (!res.ok) {
    const noti: Notification = {
      id: Date.now(),
      title: res.statusText,
      message: await res.text(),
      severity: 'error',
    }
    notificationsStore.add(noti)
    return Promise.reject(noti)
  }
  const noti: Notification = {
    id: Date.now(),
    title: 'Inventerat',
    message: `Inventering genomförd.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  console.log('Updated stock with:', body)
  return Promise.resolve()
}
