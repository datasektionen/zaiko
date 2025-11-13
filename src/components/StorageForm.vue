<template>
  <div>
    <form v-on:submit.prevent="edit ? EditStorage() : addStorage()">
      <InputText name="Namn*" :icon="InboxIcon" v-model="name" required />
      <InputDuration
        name="Intervall"
        :icon="CalendarDateRangeIcon"
        v-model="interval"
      />
      <InputCheckbox name="Skyddad" :icon="LockClosedIcon" v-model="prot" />
      <div class="submit justify-end">
        <button
          type="submit"
          class="flex items-center gap-2 p-2 bg-(--zaiko-main-color) text-(--zaiko-text-hc) rounded-md"
        >
          <DocumentCheckIcon class="w-8 aspect-square" />
          <p v-if="edit">Ändra</p>
          <p v-else>Lägg till</p>
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import InputText from '@/components/InputText.vue'
import InputCheckbox from '@/components/InputCheckbox.vue'
import { createStorage, editStorage, getStorages } from '@/stores/storageData'
import {
  type Duration,
  type StorageCreateRequest,
  type StorageEditRequest,
  type StoragesGetResponse,
} from '@/types'
import {
  CalendarDateRangeIcon,
  InboxIcon,
  LockClosedIcon,
} from '@heroicons/vue/24/outline'
import { usePopupStore } from '@/stores/popup'
import { parseISODuration, toISODuration } from '@/common'
import InputDuration from '@/components/InputDuration.vue'
import { DocumentCheckIcon } from '@heroicons/vue/16/solid'

const props = defineProps<{
  edit?: boolean
  editStorage?: StorageCreateRequest
}>()

const name = ref<string>(props.editStorage?.name || '')
const prot = ref<boolean>(props.editStorage?.protected || false)
const interval = ref<Duration>({ years: 0, months: 0, days: 0 })
if (props.editStorage?.inventory_interval) {
  interval.value = parseISODuration(props.editStorage.inventory_interval)
}

const Interval = (interval: Duration): string | undefined => {
  if (interval.years === 0 && interval.months === 0 && interval.days === 0) {
    return undefined
  }
  return toISODuration(interval)
}

const storages = ref<StoragesGetResponse>([])
getStorages().then(data => {
  storages.value = data
})

const addStorage = () => {
  const payload: StorageCreateRequest = {
    name: name.value,
    inventory_interval: Interval(interval.value),
    protected: prot.value,
  }
  createStorage(payload)
    .then(() => {
      const popupStore = usePopupStore()
      popupStore.callCurrent(payload)
      popupStore.pop()
    })
    .catch(err => {
      console.error('Error creating item:', err)
    })
}

const EditStorage = () => {
  const payload: StorageEditRequest = {
    name: props.editStorage!.name,
    inventory_interval: Interval(interval.value),
    protected: prot.value,
    new_name: name.value,
  }
  editStorage(payload)
    .then(() => {
      const popupStore = usePopupStore()
      popupStore.callCurrent(payload)
      popupStore.pop()
    })
    .catch(err => {
      console.error('Error editing storage:', err)
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
