<template>
  <div>
    <form v-on:submit.prevent="Comfirm">
      <InputText
        name="Namn*"
        :icon="ArchiveBoxIcon"
        v-model="item.new_name"
        :placeholder="item.name"
        required
      />
      <InputDuration
        name="Intervall"
        :icon="CalendarDateRangeIcon"
        v-model="interval"
      />
      <InputText
        name="Enhet*"
        :icon="LinkIcon"
        v-model="item.unit"
        :placeholder="item.unit"
        required
      />
      <div class="submit flex justify-end">
        <button
          type="submit"
          class="flex items-center gap-2 p-2 bg-(--zaiko-main-color) text-(--zaiko-text-hc) rounded-md"
        >
          <DocumentCheckIcon class="w-8 aspect-square" />
          <p>Ändra</p>
        </button>
      </div>
    </form>
  </div>
</template>

<script
  setup
  lang="ts"
  generic="T extends Record<string, any>, S extends Record<string, any>"
>
import { usePopupStore } from '@/stores/popup'
import { DocumentCheckIcon } from '@heroicons/vue/16/solid'
import {
  ArchiveBoxIcon,
  CalendarDateRangeIcon,
  LinkIcon,
} from '@heroicons/vue/24/outline'
import InputText from '@/components/InputText.vue'
import { computed, ref } from 'vue'
import InputDuration from './InputDuration.vue'
import { parseISODuration, toISODuration } from '@/common'
import type { Duration } from '@/types'

const props = defineProps<{
  item: T
  editFunc: (item: S) => Promise<void>
}>()

// HACK
console.log(props.item, 'inventory_interval' in props.item)
const interval = ref<Duration>(
  parseISODuration(
    'inventory_interval' in props.item && props.item.inventory_interval != null
      ? (props.item as any).inventory_interval
      : 'P0D',
  ),
)

const popupStore = usePopupStore()
function Comfirm() {
  let item = props.item
  if ('inventory_interval' in item) {
    ;(item as any).inventory_interval = toISODuration(interval.value)
  }
  props
    .editFunc(item as unknown as S)
    .then(() => {
      popupStore.callCurrent(item)
      popupStore.pop()
    })
    .catch(err => {
      console.error('Error editing name of item:', err)
    })
}
</script>

<style scoped></style>
