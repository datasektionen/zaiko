<template>
  <form @submit.prevent="send()" class="w-full grid grid-cols-1 gap-4 mb-4">
    <input
      type="text"
      @keyup.enter="send()"
      v-model="name"
      placeholder="Sök..."
      class="p-2 text-xl text-(--zaiko-text) border-1 border-(--zaiko-bg-2) rounded-md w-full bg-(--zaiko-bg-1)"
    />
    <div class="grid grid-cols-1 md:grid-cols-5 gap-4">
      <select
        name="Förråd"
        id="storages"
        placeholder="Förråd"
        v-model="storage"
        @keyup.enter="send()"
        class="p-2 text-xl border border-(--zaiko-bg-2) rounded-md w-full bg-(--zaiko-bg-1) text-(--zaiko-text)"
      >
        <option :value="undefined" selected class="text-(--zaiko-text)">
          Förråd
        </option>
        <option
          v-for="storage in storages"
          :value="storage.name"
          :key="storage.name"
        >
          {{ storage.name }}
        </option>
      </select>
      <select
        name="Lådor"
        id="containers"
        placeholder="Lådor"
        v-model="container"
        :disabled="!currentStorage"
        @keyup.enter="send()"
        class="p-2 text-xl border border-(--zaiko-bg-2) rounded-md w-full bg-(--zaiko-bg-1) text-(--zaiko-text) disabled:opacity-50"
      >
        <option :value="undefined" selected class="text-(--zaiko-text)">
          Låda
        </option>
        <option
          v-for="container in currentStorage"
          :value="container"
          :key="container"
        >
          {{ container === '' ? 'Ingen' : container }}
        </option>
      </select>
      <select
        name="Leverantör"
        id="supplier"
        placeholder="Leverantör"
        v-model="supplier"
        @keyup.enter="send()"
        class="p-2 text-xl border border-(--zaiko-bg-2) rounded-md w-full bg-(--zaiko-bg-1) text-(--zaiko-text)"
      >
        <option :value="undefined" selected class="text-(--zaiko-text)">
          Leverantör
        </option>
        <option
          v-for="supplier in suppliers"
          :value="supplier"
          :key="supplier.name"
        >
          {{ supplier.name }}
        </option>
      </select>
      <input
        type="number"
        v-model="min"
        placeholder="Min"
        min="0"
        @keyup.enter="send()"
        class="p-2 text-xl border border-(--zaiko-bg-2) rounded-md w-full bg-(--zaiko-bg-1) text-(--zaiko-text)"
      />
      <input
        type="number"
        v-model="max"
        placeholder="Max"
        min="1"
        @keyup.enter="send()"
        class="p-2 text-xl border border-(--zaiko-bg-2) rounded-md w-full bg-(--zaiko-bg-1) text-(--zaiko-text)"
      />
    </div>
  </form>
</template>

<script setup lang="ts">
import type {
  ItemListQueryParams,
  StorageContainersGetResponse,
  SupplierGetResponse,
} from '@/types'
import { computed, ref } from 'vue'

const name = ref<string>('')
const storage = ref<string>('')
const container = ref<string>('')
const supplier = ref<string>('')
const min = ref<number | undefined>()
const max = ref<number | undefined>()

function containerText(container: string | undefined) {
  if (container === undefined) {
    return null
  } else if (container === '') {
    return ''
  }
  return container
}

function send() {
  const payload = {
    name: name.value || null,
    storage: storage.value || null,
    container: containerText(container.value),
    min: min.value || null,
    max: max.value || null,
  } as ItemListQueryParams

  console.log('Sending filter:', payload)
  emit('send', payload)
}

const currentStorage = computed((): string[] | undefined | null => {
  const res = props.storages.find(s => s.name === storage.value)
  return res?.containers
})

const props = defineProps<{
  storages: StorageContainersGetResponse
  suppliers: SupplierGetResponse
}>()

const emit = defineEmits<{
  (e: 'send', value: ItemListQueryParams): void
}>()
</script>

<style scoped></style>
