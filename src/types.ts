export interface Item {
  id: number,
  name: string,
  location: string,
  min?: number,
  max?: number,
  current: number,
  supplier?: number,
  updated: number,
  link?: string,
}

export interface AddItem {
  name: string,
  location: string,
  min?: number,
  max?: number,
  current: number,
  supplier?: number,
  link?: string,
}

export interface UpdateItem {
  id: number,
  name: string,
  location: string,
  min?: number,
  max?: number,
  current: number,
  supplier?: number,
  link?: string,
}

export interface Supplier {
  name: string,
  link?: string,
  notes?: string,
  username?: string,
  password?: string,
  club: string,
}

export interface ShortageItem {
  name: string,
  location: string,
  min: number,
  current_amount: number,
  order_amount: number,
}

export interface Log {
  amount: number,
  time: number,
}

