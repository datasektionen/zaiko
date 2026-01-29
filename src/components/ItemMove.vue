<template>
  <div>
    <form v-on:submit.prevent="MoveItem()">
      <fieldset>
        <InputNumber
          name="Mängd*"
          :icon="ArchiveBoxIcon"
          v-model="amount"
          required
        />
      </fieldset>
      <fieldset>
        <InputSelect
          name="Nytt Förråd*"
          :icon="HomeIcon"
          v-model="storage"
          :items="storages"
          required
        >
          <template #row="item">
            <option :key="item.row.name" :value="item.row.name">
              {{ item.row.name }}
            </option>
          </template>
        </InputSelect>
        <InputSelect
          name=" Ny Låda*"
          :icon="ArchiveBoxIcon"
          v-model="container"
          :items="storages.find(i => i.name === storage)?.containers || []"
          :disabled="!storage"
        >
          <template #row="item">
            <option :key="item.row" :value="item.row">
              {{ item.row == '' ? 'Ingen' : item.row }}
            </option>
          </template>
        </InputSelect>
      </fieldset>
      <div class="submit justify-end">
        <button
          type="submit"
          class="flex items-center gap-2 p-2 bg-(--zaiko-main-color) text-(--zaiko-text-hc) rounded-md hover:cursor-pointer"
        >
          <DocumentCheckIcon class="w-8 aspect-square" />
          <p>Flytta</p>
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
  ArchiveBoxIcon,
  HomeIcon,
  DocumentCheckIcon,
} from '@heroicons/vue/16/solid'
import InputSelect from '@/components/InputSelect.vue'
import { getStorageContainers } from '@/stores/storageData'
import type { ItemMoveRequest, StorageContainersGetResponse } from '@/types'
import { moveItem } from '@/stores/itemData'
import { usePopupStore } from '@/stores/popup'
import InputNumber from './InputNumber.vue'

const props = defineProps<{
  item: {
    name: string
    storage: string
    container: string
  }
  moveFunc: (item: any) => void
}>()

const amount = ref<number>(1)
const storage = ref<string>('')
const container = ref<string>('')

const storages = ref<StorageContainersGetResponse>([])
getStorageContainers().then(data => {
  storages.value = data
})

const MoveItem = () => {
  const payload: ItemMoveRequest = {
    name: props.item.name,
    from_storage: props.item.storage,
    from_container: props.item.container,
    to_container: container.value,
    to_storage: storage.value,
    amount: amount.value,
  }
  console.log('Creating item with payload:', payload)
  moveItem(payload)
    .then(() => {
      const popupStore = usePopupStore()
      popupStore.callCurrent(payload)
      popupStore.pop()
    })
    .catch(err => {
      console.error('Error creating item:', err)
    })
}
</script>

<style scoped>
.main-content {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
  margin: 2rem auto;
}

form {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  gap: 0.5rem;
  width: 100%;
}

fieldset {
  all: unset;
  display: grid;
  grid-auto-flow: column;
  gap: 1rem;
  width: 100%;
}

.submit {
  display: flex;
  align-items: center;
  flex-direction: row;
  gap: 1rem;
}

.groupHeader {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

@media (max-width: 700px) {
  .main-content {
    margin: 0;
    gap: 0.25rem;
    max-width: 90vw;
  }

  form {
    gap: 0.5rem;
  }

  fieldset {
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }
}
</style>
