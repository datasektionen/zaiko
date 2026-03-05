<template>
  <div>
    <form @submit.prevent="submitForm">
      <fieldset>
        <InputNumber
          name="Aktuella flaskor*"
          :icon="Battery50Icon"
          v-model="form.current_bottles"
          required
          placeholder="Antal flaskor"
        />
        <InputNumber
          name="Tidigare flaskor*"
          :icon="Battery0Icon"
          v-model="form.previous_bottles"
          required
          placeholder="Tidigare antal"
        />
      </fieldset>

      <fieldset>
        <InputNumber
          name="Inköpspris (SEK)*"
          :icon="BanknotesIcon"
          v-model="form.current_purchase_price"
          required
          placeholder="Pris per flaska inkl. moms"
        />
        <InputNumber
          name="Tidigare inköpspris (SEK)"
          :icon="BanknotesIcon"
          v-model="form.previous_purchase_price"
          placeholder="Tidigare pris (frivilligt)"
        />
      </fieldset>

      <fieldset>
        <div class="p-2 bg-gray-100 rounded">
          <p class="text-sm font-semibold">Minimipris: {{ minimumSalePrice.toFixed(2) }} SEK</p>
          <p class="text-xs text-gray-600">
            = Inköpspris × 1,25
          </p>
        </div>
      </fieldset>

      <fieldset>
        <InputNumber
          name="Försäljningspris (SEK)*"
          :icon="PencilIcon"
          v-model="form.sale_price"
          placeholder="Lämna tomt för avrundning upp till 5-tal"
        />
        <p class="text-xs text-gray-600 mt-1">
          Föreslaget: {{ suggestedSalePrice.toFixed(2) }} SEK
        </p>
      </fieldset>

      <fieldset v-if="isSpirits">
        <InputNumber
          name="Pris per cl (SEK)"
          :icon="CurrencyDollarIcon"
          v-model="form.price_per_cl"
          placeholder="Frivilligt för sprit"
        />
      </fieldset>

      <div class="submit justify-end">
        <button
          type="submit"
          class="flex items-center gap-2 p-2 bg-(--zaiko-main-color) text-(--zaiko-text-hc) rounded-md hover:cursor-pointer"
        >
          <DocumentCheckIcon class="w-8 aspect-square" />
          <p>Uppdatera lager</p>
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  Battery0Icon,
  Battery50Icon,
  BanknotesIcon,
  PencilIcon,
  CurrencyDollarIcon,
  DocumentCheckIcon,
} from '@heroicons/vue/16/solid'
import InputNumber from '@/components/InputNumber.vue'
import { updateAlcoholInventory, getAlcoholProduct } from '@/stores/alcoholData'
import type {
  AlcoholProduct,
  AlcoholInventoryUpdateRequest,
} from '@/types'
import { AlcoholType } from '@/types'
import { useNotificationsStore } from '@/stores/notifications'

interface Props {
  itemName: string
  product: AlcoholProduct
}

const props = defineProps<Props>()

const emit = defineEmits<{
  success: [data: AlcoholProduct]
}>()

const notificationsStore = useNotificationsStore()

const form = ref<AlcoholInventoryUpdateRequest>({
  item_name: props.itemName,
  current_bottles: props.product.current_bottles,
  previous_bottles: props.product.previous_bottles,
  current_purchase_price: props.product.current_purchase_price,
  previous_purchase_price: props.product.previous_purchase_price || undefined,
  sale_price: props.product.sale_price,
  price_per_cl: props.product.price_per_cl || undefined,
})

const isSpirits = computed(() => props.product.alcohol_type === AlcoholType.Spirits)

const minimumSalePrice = computed(() => {
  return form.value.current_purchase_price * 1.25
})

const suggestedSalePrice = computed(() => {
  // Round up to nearest 5
  return Math.ceil(minimumSalePrice.value / 5) * 5
})

const submitForm = async () => {
  try {
    // If sale_price is not set, use suggested price
    if (!form.value.sale_price || form.value.sale_price === 0) {
      form.value.sale_price = suggestedSalePrice.value
    }

    const result = await updateAlcoholInventory(props.itemName, form.value)
    notificationsStore.addNotification({
      id: Math.random(),
      title: 'Success',
      message: 'Lager uppdaterat',
      severity: 'info',
    })
    emit('success', result)
  } catch (error) {
    notificationsStore.addNotification({
      id: Math.random(),
      title: 'Error',
      message: 'Misslyckades att uppdatera lager',
      severity: 'error',
    })
  }
}
</script>
