import type {
  SupplierGetResponse,
  Notification,
  SupplierAddRequest,
  SupplierDeleteRequest,
  SupplierEditRequest,
} from '@/types'
import { useNotificationsStore } from './notifications'

export async function getSuppliers(): Promise<SupplierGetResponse> {
  const res = await fetch('/api/supplier', { method: 'GET' })
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
  const suppliers = await res.json()
  console.log('Fetched suppliers:', suppliers)
  return Promise.resolve(suppliers)
}

export async function createSupplier(
  supplier: SupplierAddRequest,
): Promise<void> {
  const res = await fetch('/api/supplier', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(supplier),
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
    title: 'Leverantör skapad',
    message: `Leverantören "${supplier.name}" är skapad.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  return Promise.resolve()
}

export async function deleteSupplier(
  supplier: SupplierDeleteRequest,
): Promise<void> {
  const query = new URLSearchParams(supplier)
  const res = await fetch('/api/supplier?' + query.toString(), {
    method: 'DELETE',
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
    title: 'Leverantör borttagen',
    message: `Leverantören "${supplier.name}" är borttagen.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  return Promise.resolve()
}

export async function updateSupplier(
  supplier: SupplierEditRequest,
): Promise<void> {
  const res = await fetch('/api/supplier', {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(supplier),
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
    title: 'Leverantör uppdaterad',
    message: `Leverantören "${supplier.name}" är uppdaterad.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  return Promise.resolve()
}
