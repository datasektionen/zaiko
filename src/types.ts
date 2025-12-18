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
export type SupplierDeleteRequest =
  operations['delete_supplier']['parameters']['query']

export type UserInfoGetResponse =
  operations['user_info']['responses']['200']['content']['application/json']

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
