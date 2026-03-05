<template>
  <div>
    <form v-on:submit.prevent="edit ? EditContainer() : addContainer()">
      <InputText name="Namn*" :icon="ArchiveBoxIcon" v-model="name" required />
      <InputDuration
        name="Intervall"
        :icon="CalendarDateRangeIcon"
        v-model="interval"
      />
      <InputSelect
        name="Förråd*"
        :icon="HomeIcon"
        v-model="storage"
        :items="storages"
        required
        :disabled="edit"
      >
        <template #row="item">
          <option :key="item.row.name" :value="item.row.name">
            {{ item.row.name }}
          </option>
        </template>
      </InputSelect>
      <div class="submit justify-end">
        <button
          type="submit"
          class="flex items-center gap-2 p-2 bg-(--zaiko-main-color) text-(--zaiko-text-hc) rounded-md"
        >
          <DocumentCheckIcon class="w-8 aspect-square" />
          <p>Lägg till</p>
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
import {
  type ContainerCreateRequest,
  type ContainerEditRequest,
  type Duration,
  type StorageContainersGetResponse,
} from '@/types'
import { CalendarDateRangeIcon } from '@heroicons/vue/24/outline'
import { usePopupStore } from '@/stores/popup'
import { parseISODuration, toISODuration } from '@/common'
import InputDuration from './InputDuration.vue'
import { createContainer, updateContainer } from '@/stores/containerData'

const props = defineProps<{
  edit?: boolean
  editContainer?: ContainerCreateRequest
}>()

const name = ref<string>(props.editContainer?.name || '')
const storage = ref<string>(props.editContainer?.storage || '')
const interval = ref<Duration>({ years: 0, months: 0, days: 0 })
if (props.editContainer?.inventory_interval) {
  interval.value = parseISODuration(props.editContainer.inventory_interval)
}

const Interval = (interval: Duration): string | undefined => {
  if (interval.years === 0 && interval.months === 0 && interval.days === 0) {
    return undefined
  }
  return toISODuration(interval)
}

const storages = ref<StorageContainersGetResponse>([])
getStorageContainers(true).then(data => {
  storages.value = data
})

const addContainer = () => {
  const payload: ContainerCreateRequest = {
    name: name.value,
    storage: storage.value,
    inventory_interval: Interval(interval.value),
  }
  console.log('Creating container with payload:', payload)
  createContainer(payload)
    .then(() => {
      const popupStore = usePopupStore()
      popupStore.callCurrent(payload)
      popupStore.pop()
    })
    .catch(err => {
      console.error('Error creating item:', err)
    })
}

const EditContainer = () => {
  console.log('Editing container...')
  const payload: ContainerEditRequest = {
    name: props.editContainer!.name,
    new_name: name.value,
    storage: storage.value,
  }
  updateContainer(payload)
    .then(() => {
      const popupStore = usePopupStore()
      popupStore.callCurrent(payload)
      popupStore.pop()
    })
    .catch(err => {
      console.error('Error editing container:', err)
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
