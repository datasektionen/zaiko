import type { operations, components } from './openapi'
import type { Component, FunctionalComponent } from 'vue'

export type ItemGetResponse =
  operations['get_item']['responses']['200']['content']['application/json']
export type ItemAddRequest = components['schemas']['ItemAddRequest']
export type ItemDeleteRequest = operations['delete_item']['parameters']['query']
export type ItemListGetResponse =
  operations['get_items']['responses']['200']['content']['application/json']
export type ItemListQueryParams = operations['get_items']['parameters']['query']
export type ItemLinkSupplierRequest =
  operations['supply_item']['requestBody']['content']['application/json']
export type ItemUnlinkSupplierRequest =
  operations['unsupply_item']['parameters']['query']
export type ItemEditLinkSupplierRequest =
  operations['change_supply_item']['requestBody']['content']['application/json']
export type ItemEditRequest =
  operations['change_item']['requestBody']['content']['application/json']
export type ItemStorageEditRequest =
  operations['change_stored_item']['requestBody']['content']['application/json']
export type ItemMoveRequest =
  operations['move_item']['requestBody']['content']['application/json']

export type StateEnum = components['schemas']['OrderState']

export type StorageContainersGetResponse =
  operations['get_storage_container_tree']['responses']['200']['content']['application/json']
export type StorageTreeGetResponse =
  operations['get_storage_container_tree']['responses']['200']['content']['application/json']
export type StorageTreeRequest =
  operations['get_container_item_tree_for_storage']['parameters']['query']
export type StorageContainersTreeGetResponse =
  operations['get_container_item_tree_for_storage']['responses']['200']['content']['application/json']
export type TreeNode =
  operations['get_container_item_tree_for_storage']['responses']['200']['content']['application/json'][0]
export type MinimalItem = components['schemas']['MinimalItem']
export type StorageCreateRequest = components['schemas']['StorageCreateRequest']
export type StorageEditRequest =
  operations['change_storage']['requestBody']['content']['application/json']
export type StorageDeleteRequest =
  operations['destroy_storage']['parameters']['query']
export type StoragesGetResponse =
  operations['get_storages']['responses']['200']['content']['application/json']

export type ContainerCreateRequest =
  operations['create_container']['requestBody']['content']['application/json']
export type ContainerDeleteRequest =
  operations['destroy_container']['parameters']['query']
export type ContainerMoveRequest =
  operations['move_container']['requestBody']['content']['application/json']
export type ContainerEditRequest =
  operations['change_container']['requestBody']['content']['application/json']

export type StatsGetResponse =
  operations['get_stats']['responses']['200']['content']['application/json']

export type LogGetResponse =
  operations['get_log']['responses']['200']['content']['application/json']
export type LogQueryParams = operations['get_log']['parameters']['query']

export type ShortageGetResponse =
  operations['get_shortage']['responses']['200']['content']['application/json']
export type StockTreeGetResponse =
  operations['items_due']['responses']['200']['content']['application/json']
export type StockUpdateRequest =
  operations['take_stock']['requestBody']['content']['application/json']
export type StockUpdateItem = components['schemas']['StockUpdate']

export type SupplierGetResponse =
  operations['get_suppliers']['responses']['200']['content']['application/json']
export type SupplierAddRequest =
  operations['create_supplier']['requestBody']['content']['application/json']
export type SupplierEditRequest =
  operations['update_supplier']['requestBody']['content']['application/json']
export type SupplierDeleteRequest =
  operations['delete_supplier']['parameters']['query']

export type UserInfoGetResponse =
  operations['user_info']['responses']['200']['content']['application/json']

export type HivePermission = components['schemas']['HivePermission']

// Alcohol Types
export enum AlcoholType {
  Cider = 'cider',
  Beer = 'beer',
  Spirits = 'spirits',
  Wine = 'wine',
}

export interface AlcoholProduct {
  item_name: string
  alcohol_type: AlcoholType
  volume_cl: number
  supplier: string
  current_bottles: number
  previous_bottles: number
  current_purchase_price: number
  previous_purchase_price: number | null
  minimum_sale_price: number
  sale_price: number
  price_per_cl: number | null
  last_updated: string | null
}

export interface AlcoholProductCreateRequest {
  item_name: string
  alcohol_type: AlcoholType
  volume_cl: number
  supplier: string
}

export interface AlcoholInventoryUpdateRequest {
  item_name: string
  current_bottles: number
  previous_bottles: number
  current_purchase_price: number
  previous_purchase_price?: number
  sale_price?: number
  price_per_cl?: number
}

export interface AlcoholReportEntry {
  item_name: string
  alcohol_type: AlcoholType
  supplier: string
  volume_cl: number
  current_bottles: number
  previous_bottles: number
  bottle_change: number
  current_total_value: number
  previous_total_value: number
  value_change: number
  current_purchase_price: number
  previous_purchase_price: number | null
  sale_price: number
}

export interface AlcoholTypeSummary {
  alcohol_type: AlcoholType
  count: number
  total_bottles: number
  total_value: number
  value_change: number
}

export interface AlcoholInventoryReport {
  report_date: string
  entries: AlcoholReportEntry[]
  summary_by_type: AlcoholTypeSummary[]
  total_value: number
}

export type Duration = {
  years?: number
  months?: number
  weeks?: number
  days?: number
  hours?: number
  minutes?: number
  seconds?: number
}

export interface PopupItem {
  component: Component
  props?: Record<string, any>
  title: string
  icon: FunctionalComponent
  cb?: (result: any) => any
}

export interface FilterItemParams {
  column: string
  search: string
}

export interface Notification {
  id: number
  title: string
  message: string
  severity: 'error' | 'warning' | 'info'
}
