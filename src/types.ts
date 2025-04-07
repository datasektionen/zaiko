import type { FunctionalComponent } from 'vue';

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

export interface FilterColumn {
  name: string,
  label: string,
  icon: FunctionalComponent,
}

export interface FilterItemParams {
  column: string,
  search: string,
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
  supplier: string,
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

export interface TableColumn {
  value: string,
  label: string,
  icon: FunctionalComponent,
}

export interface Notification {
  id: number,
  title: string,
  message: string,
  severity: "error" | "warning" | "info",
}

export interface ClubStorage {
  club: ClubGetRequest,
  clubs: Array<ClubGetRequest>,
  timestamp: number,
}

export interface ClubGetRequest {
  name: string,
  permission: "r" | "rw",
}

export interface Stats {
  items: number,
  suppliers: number,
  shortages: number,
}
