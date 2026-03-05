<template>
  <div>
    <form @submit.prevent="submitForm">
      <fieldset>
        <InputText
          name="Produkt*"
          :icon="ArchiveBoxIcon"
          v-model="form.item_name"
          required
          :disabled="edit"
          placeholder="e.g., Staropramen"
        />
        <InputSelect
          name="Alkohotyp*"
          :icon="BeakerIconSolid"
          v-model="form.alcohol_type"
          :items="alcoholTypes"
          required
          :disabled="edit"
        >
          <template #row="item">
            <option :key="item.row" :value="item.row">
              {{ formatAlcoholType(item.row) }}
            </option>
          </template>
        </InputSelect>
      </fieldset>
      <fieldset>
        <InputNumber
          name="Volym (cl)*"
          :icon="Square3Stack3DIcon"
          v-model="form.volume_cl"
          required
          :disabled="edit"
          placeholder="e.g., 33, 50, 70"
        />
        <InputSelect
          name="Leverantör*"
          :icon="ShoppingCartIcon"
          v-model="form.supplier"
          :items="suppliers"
          required
        >
          <template #row="item">
            <option :key="item.row.name" :value="item.row.name">
              {{ item.row.name }}
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
          <p v-if="edit">Ändra</p>
          <p v-else>Lägg till</p>
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import {
  ArchiveBoxIcon,
  DocumentCheckIcon,
  BeakerIcon as BeakerIconSolid,
  Square3Stack3DIcon,
  ShoppingCartIcon,
} from '@heroicons/vue/16/solid'
import InputText from '@/components/InputText.vue'
import InputNumber from '@/components/InputNumber.vue'
import InputSelect from '@/components/InputSelect.vue'
import { getSuppliers } from '@/stores/supplierData'
import { createAlcoholProduct } from '@/stores/alcoholData'
import type {
  AlcoholType,
  AlcoholProductCreateRequest,
  SupplierGetResponse,
} from '@/types'
import { AlcoholType as AlcoholTypeEnum } from '@/types'
import { useNotificationsStore } from '@/stores/notifications'

interface Props {
  edit?: boolean
  initialData?: AlcoholProductCreateRequest
}

const props = withDefaults(defineProps<Props>(), {
  edit: false,
})

const emit = defineEmits<{
  success: [data: any]
}>()

const suppliers = ref<SupplierGetResponse>([])
const notificationsStore = useNotificationsStore()

const alcoholTypes = Object.values(AlcoholTypeEnum)

const form = ref<AlcoholProductCreateRequest>({
  item_name: '',
  alcohol_type: AlcoholTypeEnum.Beer,
  volume_cl: 0,
  supplier: '',
})

const formatAlcoholType = (type: AlcoholType): string => {
  const labels: Record<AlcoholType, string> = {
    cider: 'Cider',
    beer: 'Öl',
    spirits: 'Sprit',
    wine: 'Vin',
  }
  return labels[type] || type
}

onMounted(async () => {
  try {
    suppliers.value = await getSuppliers()
    if (props.initialData) {
      form.value = { ...props.initialData }
    }
  } catch (error) {
    notificationsStore.addNotification({
      id: Math.random(),
      title: 'Error',
      message: 'Failed to load suppliers',
      severity: 'error',
    })
  }
})

const submitForm = async () => {
  try {
    const result = await createAlcoholProduct(form.value)
    notificationsStore.addNotification({
      id: Math.random(),
      title: 'Success',
      message: props.edit ? 'Alkoholprodukt uppdaterad' : 'Alkoholprodukt skapad',
      severity: 'info',
    })
    emit('success', result)
  } catch (error) {
    notificationsStore.addNotification({
      id: Math.random(),
      title: 'Error',
      message: 'Failed to save alcohol product',
      severity: 'error',
    })
  }
}
</script>
