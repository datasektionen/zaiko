<template>
  <div>
    <div>
      <div class="flex gap-4 justify-center mt-4">
        <button
          @click="Comfirm"
          class="bg-(--zaiko-bad-color) p-2 gap-2 flex items-center text-(--zaiko-text-hc) rounded-md cursor-pointer"
        >
          <BackspaceIcon class="w-6 h-6 inline text-(--zaiko-text-hc)" />
          <p>Ta bort</p>
        </button>
        <button
          @click="Return"
          class="bg-(--zaiko-text) flex items-center gap-2 p-2 text-(--zaiko-text-hc) rounded-md cursor-pointer"
        >
          <ArrowUturnLeftIcon class="w-6 h-6 inline text-(--zaiko-text-hc)" />
          <p>Tillbaka</p>
        </button>
      </div>
    </div>
  </div>
</template>

<script
  setup
  lang="ts"
  generic="T extends Record<string, any>, S extends Record<string, any>"
>
import { usePopupStore } from '@/stores/popup'
import { BackspaceIcon, ArrowUturnLeftIcon } from '@heroicons/vue/24/outline'
const props = defineProps<{
  item: T
  deleteFunc: (item: S) => Promise<void>
}>()

const popupStore = usePopupStore()

function Comfirm() {
  props
    .deleteFunc(props.item as unknown as S)
    .then(() => {
      popupStore.callCurrent(props.item)
      popupStore.pop()
    })
    .catch(err => {
      console.error('Error deleting item:', err)
    })
}

function Return() {
  popupStore.pop()
}
</script>

<style scoped></style>
