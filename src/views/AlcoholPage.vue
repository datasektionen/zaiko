<template>
  <div class="space-y-6">
    <TitleBig :text="'Alkoholhantering'" :icon="BeakerIcon" />

    <div class="tabs flex gap-2 border-b">
      <button
        @click="activeTab = 'list'"
        :class="[
          'px-4 py-2 font-semibold transition',
          activeTab === 'list'
            ? 'text-(--zaiko-main-color) border-b-2 border-(--zaiko-main-color)'
            : 'text-gray-600',
        ]"
      >
        Produkter
      </button>
      <button
        @click="activeTab = 'add'"
        :class="[
          'px-4 py-2 font-semibold transition',
          activeTab === 'add'
            ? 'text-(--zaiko-main-color) border-b-2 border-(--zaiko-main-color)'
            : 'text-gray-600',
        ]"
      >
        Lägg till produkt
      </button>
      <button
        @click="activeTab = 'report'"
        :class="[
          'px-4 py-2 font-semibold transition',
          activeTab === 'report'
            ? 'text-(--zaiko-main-color) border-b-2 border-(--zaiko-main-color)'
            : 'text-gray-600',
        ]"
      >
        Rapport
      </button>
    </div>

    <!-- Products List Tab -->
    <div v-if="activeTab === 'list'" class="space-y-4">
      <div v-if="loading" class="text-center py-8">
        <p>Laddar produkter...</p>
      </div>
      <div v-else-if="products.length === 0" class="text-center py-8 text-gray-500">
        <p>Inga alkoholprodukter hittades</p>
      </div>
      <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div
          v-for="product in products"
          :key="product.item_name"
          class="p-4 border rounded-lg shadow hover:shadow-lg transition cursor-pointer"
          @click="selectProduct(product)"
        >
          <div class="flex justify-between items-start mb-3">
            <div>
              <h3 class="text-lg font-bold">{{ product.item_name }}</h3>
              <p class="text-sm text-gray-600">{{ formatAlcoholType(product.alcohol_type) }}</p>
            </div>
            <button
              @click.stop="deleteProduct(product.item_name)"
              class="text-red-600 hover:text-red-800 p-1"
              :title="'Ta bort ' + product.item_name"
            >
              <TrashIcon class="w-5 aspect-square" />
            </button>
          </div>

          <div class="grid grid-cols-2 gap-2 text-sm mb-3">
            <div>
              <p class="text-gray-600">Produkt-ID</p>
              <p class="font-semibold">{{ product.product_id || '—' }}</p>
            </div>
            <div>
              <p class="text-gray-600">Volym</p>
              <p class="font-semibold">{{ product.volume_cl }} cl</p>
            </div>
            <div>
              <p class="text-gray-600">Leverantör</p>
              <p class="font-semibold">{{ product.supplier || '—' }}</p>
            </div>
            <div>
              <p class="text-gray-600">Flaskor</p>
              <p class="font-semibold">{{ product.current_bottles }}</p>
            </div>
            <div>
              <p class="text-gray-600">Försäljningspris</p>
              <p class="font-semibold">{{ formatCurrency(product.sale_price) }} SEK</p>
            </div>
          </div>

          <button
            @click.stop="openInventoryForm(product)"
            class="w-full p-2 bg-(--zaiko-main-color) text-(--zaiko-text-hc) rounded hover:opacity-90 transition text-sm font-semibold"
          >
            Uppdatera lager
          </button>
        </div>
      </div>
    </div>

    <!-- Add Product Tab -->
    <div v-if="activeTab === 'add'" class="max-w-md mx-auto">
      <PanelTemplate title="Lägg till alkoholprodukt">
        <AlcoholForm @success="onProductAdded" />
      </PanelTemplate>
    </div>

    <!-- Report Tab -->
    <div v-if="activeTab === 'report'">
      <AlcoholReport />
    </div>

    <!-- Inventory Update Modal -->
    <PopupModal
      v-if="selectedProduct && showInventoryForm"
      title="Uppdatera lager"
      @close="showInventoryForm = false"
    >
      <AlcoholInventoryForm
        :item-name="selectedProduct.item_name"
        :product="selectedProduct"
        @success="onInventoryUpdated"
      />
    </PopupModal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { BeakerIcon, TrashIcon } from '@heroicons/vue/16/solid'
import TitleBig from '@/components/TitleBig.vue'
import PanelTemplate from '@/components/PanelTemplate.vue'
import PopupModal from '@/components/PopupModal.vue'
import AlcoholForm from '@/components/AlcoholForm.vue'
import AlcoholInventoryForm from '@/components/AlcoholInventoryForm.vue'
import AlcoholReport from '@/components/AlcoholReport.vue'
import {
  getAlcoholProducts,
  deleteAlcoholProduct,
} from '@/stores/alcoholData'
import type { AlcoholProduct, AlcoholType } from '@/types'
import { useNotificationsStore } from '@/stores/notifications'

const notificationsStore = useNotificationsStore()

const activeTab = ref<'list' | 'add' | 'report'>('list')
const products = ref<AlcoholProduct[]>([])
const selectedProduct = ref<AlcoholProduct | null>(null)
const showInventoryForm = ref(false)
const loading = ref(false)

const formatAlcoholType = (type: AlcoholType): string => {
  const labels: Record<AlcoholType, string> = {
    cider: 'Cider',
    beer: 'Öl',
    spirits: 'Sprit',
    wine: 'Vin',
  }
  return labels[type] || type
}

const formatCurrency = (value: number): string => {
  return value.toFixed(2)
}

const loadProducts = async () => {
  loading.value = true
  try {
    products.value = await getAlcoholProducts()
  } catch (error) {
    notificationsStore.addNotification({
      id: Math.random(),
      title: 'Error',
      message: 'Misslyckades att ladda alkoholprodukter',
      severity: 'error',
    })
  } finally {
    loading.value = false
  }
}

const selectProduct = (product: AlcoholProduct) => {
  selectedProduct.value = product
}

const openInventoryForm = (product: AlcoholProduct) => {
  selectedProduct.value = product
  showInventoryForm.value = true
}

const deleteProduct = async (itemName: string) => {
  if (confirm(`Vill du ta bort ${itemName}?`)) {
    try {
      await deleteAlcoholProduct(itemName)
      notificationsStore.addNotification({
        id: Math.random(),
        title: 'Success',
        message: 'Produkt borttagen',
        severity: 'info',
      })
      await loadProducts()
    } catch (error) {
      notificationsStore.addNotification({
        id: Math.random(),
        title: 'Error',
        message: 'Misslyckades att ta bort produkt',
        severity: 'error',
      })
    }
  }
}

const onProductAdded = async () => {
  showInventoryForm.value = false
  activeTab.value = 'list'
  await loadProducts()
}

const onInventoryUpdated = async () => {
  showInventoryForm.value = false
  await loadProducts()
}

onMounted(() => {
  loadProducts()
})
</script>
