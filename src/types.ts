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

export interface ShortageItem {
  id: number,
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

