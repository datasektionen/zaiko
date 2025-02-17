
export interface ItemGetResponse {
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

export interface ItemAddRequest {
  name: string,
  location: string,
  min?: number,
  max?: number,
  current: number,
  supplier?: number,
  link?: string,
}

export interface ItemUpdateRequest {
  id: number,
  name: string,
  location: string,
  min?: number,
  max?: number,
  current: number,
  supplier?: number,
  link?: string,
}

export interface SupplierGetResponse {
  id: number,
  name: string,
  link?: string,
  notes?: string,
  username?: string,
  password?: string,
  updated: number
}

export interface SupplierAddRequest {
  name: string,
  link?: string,
  notes?: string,
  username?: string,
  password?: string,
}

export interface SupplierUpdateRequest {
  id: number,
  name: string,
  link?: string,
  notes?: string,
  username?: string,
  password?: string,
}

export interface StockGetResponse {
  id: number,
  name: string,
  location: string,
  min: number,
  current: number,
  order: number,
}

export interface StockUpdateRequest {
  items: Array<Array<number>>
}

export interface Log {
  amount: number,
  time: number,
}

export interface SupplierListGetResponse {
    id: number,
    name: string,
}

export interface Notification {
  id: number,
  title: string,
  message: string,
  severity: string,
}

export interface ClubStorage {
  club: string,
  clubs: Array<string>,
  timestamp: number,
}
