<template>
  <div>
    <form v-on:submit.prevent="MoveContainer()">
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
      <InputCheckbox name="Merge" :icon="FunnelIcon" v-model="merge" />
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
import InputText from '@/components/InputText.vue'
import InputSelect from '@/components/InputSelect.vue'
import { getStorageContainers } from '@/stores/storageData'
import { moveContainer } from '@/stores/containerData'
import type {
  ContainerMoveRequest,
  StorageContainersGetResponse,
} from '@/types'
import { usePopupStore } from '@/stores/popup'
import InputCheckbox from './InputCheckbox.vue'
import { FunnelIcon } from '@heroicons/vue/24/solid'

const props = defineProps<{
  container: {
    name: string
    storage: string
  }
  moveFunc: (item: any) => void
}>()

const name = ref<string>(props.container.name)
const merge = ref<boolean>(false)
const storage = ref<string>('')

const storages = ref<StorageContainersGetResponse>([])
getStorageContainers().then(data => {
  storages.value = data
})

const MoveContainer = () => {
  const payload: ContainerMoveRequest = {
    name: name.value,
    from_storage: props.container.storage,
    to_storage: storage.value,
    merge: merge.value,
  }
  console.log('Creating item with payload:', payload)
  moveContainer(payload)
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
