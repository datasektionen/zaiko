import type {
  ItemAddRequest,
  ItemDeleteRequest,
  ItemGetResponse,
  ItemLinkSupplierRequest,
  ItemListGetResponse,
  ItemListQueryParams,
  ItemEditRequest,
  ItemUnlinkSupplierRequest,
  Notification,
  ItemStorageEditRequest,
  ItemMoveRequest,
  ItemEditLinkSupplierRequest,
} from '@/types'
import { useNotificationsStore } from './notifications'
import { fetchOrRedirect } from '@/common'

export async function getItemByName(name: string): Promise<ItemGetResponse> {
  const query = new URLSearchParams({ name: name })
  const res = await fetchOrRedirect('/api/item?' + query.toString(), {
    method: 'GET',
  })
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
  const item = await res.json()
  console.log('Fetched item:', item)
  return Promise.resolve(item)
}

// Iterate over the keys of params and add them to queryParams if they are defined
function buildQueryParams(params: ItemListQueryParams): URLSearchParams {
  const queryParams = new URLSearchParams()

  Object.keys(params!).forEach(key => {
    const value = (params as any)[key]
    if (value !== undefined && value !== null) {
      if (Array.isArray(value)) {
        value.forEach(v => queryParams.append(key, v.toString()))
      } else {
        queryParams.append(key, value.toString())
      }
    }
  })

  // Can't filter on container without storage
  if (queryParams.has('container') && !queryParams.has('storage')) {
    queryParams.delete('container')
  }

  return queryParams
}

export async function getItems(
  query?: ItemListQueryParams,
): Promise<ItemListGetResponse> {
  let requestString = '/api/items'
  if (query) {
    const queryParams = buildQueryParams(query)
    requestString += '?' + queryParams.toString()
  }
  const res = await fetch(requestString, { method: 'GET' })

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
  const items = await res.json()
  console.log('Fetched items:', items)
  return Promise.resolve(items)
}

export async function createItem(item: ItemAddRequest): Promise<void> {
  const res = await fetch('/api/item', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(item),
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
    title: 'Produkt skapat',
    message: `"${item.name}" är skapad.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  console.log('Created item:', item)
  return Promise.resolve()
}

export async function moveItem(item: ItemMoveRequest): Promise<void> {
  const res = await fetch('/api/item/move', {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(item),
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
    title: 'Produkt flyttad',
    message: `"${item.name}" har flyttats till "${item.to_container}" i "${item.to_storage}".`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  console.log('Moved item:', item)
  return Promise.resolve()
}

export async function deleteItem(item: ItemDeleteRequest): Promise<void> {
  const query = new URLSearchParams(item)
  const res = await fetch('/api/item?' + query.toString(), { method: 'DELETE' })
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
    title: 'Produkt borttagen',
    message: `"${item.name}" är borttagen.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  console.log('Item deleted:', item.name)
  return Promise.resolve()
}

export async function supplierLinkItem(
  link: ItemLinkSupplierRequest,
): Promise<void> {
  const res = await fetch('/api/supply', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(link),
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
    title: 'Leverantör länkad',
    message: `"${link.supplier}" är länkad till "${link.name}".`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  console.log('Linked supplier to item:', link)
  return Promise.resolve()
}

export async function supplierUnlinkItem(
  link: ItemUnlinkSupplierRequest,
): Promise<void> {
  const query = new URLSearchParams(link)
  const res = await fetch('/api/supply?' + query.toString(), {
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
    title: 'Leverantör avlänkad',
    message: `"${link.supplier}" är avlänkad från "${link.name}".`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  console.log('Unlinked supplier from item:', link)
  return Promise.resolve()
}

export async function supplierEditLinkItem(
  link: ItemEditLinkSupplierRequest,
): Promise<void> {
  const res = await fetch('/api/supply', {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(link),
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
    title: 'Leverantör länk uppdaterad',
    message: `"${link.supplier}" är uppdaterad för "${link.name}".`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  console.log('Edited supplier link for item:', link)
  return Promise.resolve()
}

export async function changeItemName(body: ItemEditRequest): Promise<void> {
  const res = await fetch('/api/item', {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
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
    title: 'Produkt namn ändrat',
    message: `"${body.name}" är nu "${body.new_name}".`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  console.log('Changed item name from', body.name, 'to', body.new_name)
  return Promise.resolve()
}

export async function editItemStorage(
  body: ItemStorageEditRequest,
): Promise<void> {
  const res = await fetch('/api/item', {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
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
    title: 'Produkt ändrad',
    message: `"${body.name}" har updaterats.`,
    severity: 'info',
  }
  notificationsStore.add(noti)
  console.log('Edited item', body.name)
  return Promise.resolve()
}
