import type {
  ContainerCreateRequest,
  ContainerDeleteRequest,
  ContainerEditRequest,
  ContainerMoveRequest,
  Notification,
} from '@/types'
import { useNotificationsStore } from './notifications'

export async function createContainer(
  container: ContainerCreateRequest,
): Promise<void> {
  const res = await fetch('/api/container', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(container),
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
    title: 'Låda skapad',
    message: `Låda "${container.name}" är skapad.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  return Promise.resolve()
}

export async function deleteContainer(
  container: ContainerDeleteRequest,
): Promise<void> {
  const query = new URLSearchParams(container)
  const res = await fetch('/api/container?' + query.toString(), {
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
    title: 'Låda borttagen',
    message: `Lådan "${container.name}" är borttagen.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  return Promise.resolve()
}

export async function updateContainer(
  container: ContainerEditRequest,
): Promise<void> {
  const res = await fetch('/api/container', {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(container),
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
    title: 'Låda uppdaterad',
    message: `Låda "${container.name}" är uppdaterad.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  return Promise.resolve()
}

export async function moveContainer(
  container: ContainerMoveRequest,
): Promise<void> {
  const res = await fetch('/api/container/move', {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(container),
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
    title: 'Låda flyttad',
    message: `Låda "${container.name}" är flyttad.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  return Promise.resolve()
}
