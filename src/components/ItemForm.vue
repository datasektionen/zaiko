<template>
  <div>
    <form v-on:submit.prevent="edit ? EditItem() : addItem()">
      <fieldset>
        <InputText name="Produkt*" :icon="ArchiveBoxIcon" v-model="name" required :disabled="edit" />
        <InputText name="Enhet" :icon="LinkIcon" v-model="unit" placeholder="e.g; 3-pack, %, 2L" :disabled="edit" />
      </fieldset>
      <InputDuration name="Intervall" :icon="CalendarDateRangeIcon" v-model="interval" :disabled="edit" />
      <fieldset>
        <InputNumber name="Min" :icon="Battery0Icon" v-model="min" />
        <InputNumber name="Max" :icon="Battery100Icon" v-model="max" />
        <InputNumber name="Nuvarande*" :icon="Battery50Icon" v-model="current" required />
      </fieldset>
      <fieldset>
        <InputSelect name="Förråd*" :icon="HomeIcon" v-model="storage" :items="storages" required>
          <template #row="item">
            <option :key="item.row.name" :value="item.row.name">
              {{ item.row.name }}
            </option>
          </template>
        </InputSelect>
        <InputSelect name="Låda*" :icon="ArchiveBoxIcon" v-model="container"
          :items="storages.find((i) => i.name === storage)?.containers || []" :disabled="!storage">
          <template #row="item">
            <option :key="item.row" :value="item.row">
              {{ item.row == "" ? "Ingen" : item.row }}
            </option>
          </template>
        </InputSelect>
      </fieldset>
      <div class="submit justify-end">
        <button type="submit"
          class="flex items-center gap-2 p-2 bg-(--zaiko-main-color) text-(--zaiko-text-hc) rounded-md hover:cursor-pointer">
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
import { ArchiveBoxIcon, HomeIcon, LinkIcon, DocumentCheckIcon, Battery0Icon, Battery100Icon, Battery50Icon } from '@heroicons/vue/16/solid';
import InputText from '@/components/InputText.vue';
import InputNumber from '@/components/InputNumber.vue';
import InputSelect from '@/components/InputSelect.vue';
import { getStorageContainers } from '@/stores/storageData';
import type { Duration, ItemAddRequest, ItemStorageEditRequest, Notification, StorageContainersGetResponse} from '@/types';
import { CalendarDateRangeIcon } from '@heroicons/vue/24/outline';
import { createItem, editItemStorage } from '@/stores/itemData';
import { usePopupStore } from '@/stores/popup';
import { forceNumeric, parseISODuration, toISODuration } from '@/common';
import InputDuration from './InputDuration.vue';
import { useNotificationsStore } from '@/stores/notifications';

const props = defineProps<{
  edit?: boolean,
  editItem?: ItemAddRequest,
}>()

const name = ref<string>(props.editItem?.name || "")
const unit = ref<string | undefined>(props.editItem?.unit || undefined)
const storage = ref<string>(props.editItem?.storage || "")
const container = ref<string>(props.editItem?.container || "")
const min = ref<number | undefined>(props.editItem?.min || undefined)
const max = ref<number | undefined>(props.editItem?.max || undefined)
const current = ref<number>(props.editItem?.amount || 0)
const interval = ref<Duration>({ years: 0, months: 0, days: 0 })
if (props.editItem?.inventory_interval) {
  interval.value = parseISODuration(props.editItem.inventory_interval)
}

const Interval = (interval: Duration): string | undefined => {
  if (interval.years === 0 && interval.months === 0 && interval.days === 0) {
    return undefined
  }
  return toISODuration(interval)
}

const storages = ref<StorageContainersGetResponse>([])
getStorageContainers().then((data) => {
  storages.value = data
})

const addItem = () => {
  const payload: ItemAddRequest = {
    name: name.value,
    unit: unit.value ? unit.value : undefined,
    storage: storage.value,
    container: container.value,
    min: forceNumeric(min.value),
    max: forceNumeric(max.value),
    amount: forceNumeric(current.value) as number,
    inventory_interval: Interval(interval.value),
  }
  console.log("Creating item with payload:", payload);
  createItem(payload).then(() => {
    const popupStore = usePopupStore();
    popupStore.callCurrent(payload);
    popupStore.pop();
  }).catch((err) => {
    console.error("Error creating item:", err);
  })
}

const EditItem = () => {
  const payload: ItemStorageEditRequest = {
    name: props.editItem!.name,
    storage: props.editItem!.storage,
    container: props.editItem!.container,
    min: min.value,
    max: max.value,
    amount: current.value,
    new_container: container.value,
    new_storage: storage.value,
  }
  editItemStorage(payload).then(() => {
    const popupStore = usePopupStore();
    popupStore.callCurrent(payload);
    popupStore.pop();
  }).catch((err) => {
    console.error("Error editing item:", err);
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
