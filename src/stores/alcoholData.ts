import type {
  AlcoholProduct,
  AlcoholProductCreateRequest,
  AlcoholInventoryUpdateRequest,
  AlcoholInventoryReport,
  Notification,
} from '@/types'
import { useNotificationsStore } from './notifications'
import { fetchOrRedirect } from '@/common'

export async function createAlcoholProduct(
  data: AlcoholProductCreateRequest,
): Promise<AlcoholProduct> {
  const notificationsStore = useNotificationsStore()

  const res = await fetchOrRedirect('/api/alcohol', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  })

  if (!res.ok) {
    const noti: Notification = {
      id: Math.random(),
      title: 'Error',
      message: 'Failed to create alcohol product',
      severity: 'error',
    }
    notificationsStore.addNotification(noti)
    throw new Error('Failed to create alcohol product')
  }

  return res.json()
}

export async function getAlcoholProducts(): Promise<AlcoholProduct[]> {
  const res = await fetchOrRedirect('/api/alcohol', {
    method: 'GET',
  })

  if (!res.ok) {
    throw new Error('Failed to fetch alcohol products')
  }

  return res.json()
}

export async function getAlcoholProduct(itemName: string): Promise<AlcoholProduct> {
  const res = await fetchOrRedirect(`/api/alcohol/${itemName}`, {
    method: 'GET',
  })

  if (!res.ok) {
    throw new Error('Failed to fetch alcohol product')
  }

  return res.json()
}

export async function updateAlcoholInventory(
  itemName: string,
  data: AlcoholInventoryUpdateRequest,
): Promise<AlcoholProduct> {
  const notificationsStore = useNotificationsStore()

  const res = await fetchOrRedirect(`/api/alcohol/${itemName}/inventory`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  })

  if (!res.ok) {
    const noti: Notification = {
      id: Math.random(),
      title: 'Error',
      message: 'Failed to update alcohol inventory',
      severity: 'error',
    }
    notificationsStore.addNotification(noti)
    throw new Error('Failed to update alcohol inventory')
  }

  return res.json()
}

export async function deleteAlcoholProduct(itemName: string): Promise<void> {
  const notificationsStore = useNotificationsStore()

  const res = await fetchOrRedirect(`/api/alcohol/${itemName}`, {
    method: 'DELETE',
  })

  if (!res.ok) {
    const noti: Notification = {
      id: Math.random(),
      title: 'Error',
      message: 'Failed to delete alcohol product',
      severity: 'error',
    }
    notificationsStore.addNotification(noti)
    throw new Error('Failed to delete alcohol product')
  }
}

export async function getAlcoholReport(): Promise<AlcoholInventoryReport> {
  const res = await fetchOrRedirect('/api/alcohol/report', {
    method: 'GET',
  })

  if (!res.ok) {
    throw new Error('Failed to fetch alcohol report')
  }

  return res.json()
}
