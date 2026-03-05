<template>
  <div class="p-4">
    <div class="mb-6">
      <button
        @click="loadReport"
        class="flex items-center gap-2 p-2 bg-(--zaiko-main-color) text-(--zaiko-text-hc) rounded-md hover:cursor-pointer"
      >
        <ArrowDownTrayIcon class="w-8 aspect-square" />
        Ladda rapport
      </button>
    </div>

    <div v-if="report" class="space-y-6">
      <div class="mb-4">
        <h2 class="text-2xl font-bold">
          Alkoholrapport - {{ formatDate(report.report_date) }}
        </h2>
        <p class="text-lg font-semibold text-(--zaiko-main-color)">
          Totalt värde: {{ formatCurrency(report.total_value) }} SEK
        </p>
      </div>

      <!-- Summary by type -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div
          v-for="summary in report.summary_by_type"
          :key="summary.alcohol_type"
          class="p-4 border rounded-lg"
        >
          <h3 class="text-lg font-semibold mb-3">
            {{ formatAlcoholType(summary.alcohol_type) }}
          </h3>
          <div class="space-y-2 text-sm">
            <p><span class="font-semibold">Produkter:</span> {{ summary.count }}</p>
            <p><span class="font-semibold">Totalt flaskor:</span> {{ summary.total_bottles }}</p>
            <p><span class="font-semibold">Värde:</span> {{ formatCurrency(summary.total_value) }} SEK</p>
            <p :class="['font-semibold', summary.value_change >= 0 ? 'text-green-600' : 'text-red-600']">
              Förändring: {{ formatCurrency(summary.value_change) }} SEK
              ({{ formatPercentage(summary.value_change, summary.total_value) }}%)
            </p>
          </div>
        </div>
      </div>

      <!-- Detailed entries -->
      <div class="mt-8">
        <h3 class="text-xl font-bold mb-4">Detaljer</h3>
        <div class="overflow-x-auto">
          <table class="w-full border-collapse border border-gray-300">
            <thead class="bg-gray-200">
              <tr>
                <th class="border border-gray-300 p-2 text-left">Produkt</th>
                <th class="border border-gray-300 p-2 text-left">Typ</th>
                <th class="border border-gray-300 p-2 text-left">Leverantör</th>
                <th class="border border-gray-300 p-2 text-right">Volym (cl)</th>
                <th class="border border-gray-300 p-2 text-right">Flaskor</th>
                <th class="border border-gray-300 p-2 text-right">Förändring</th>
                <th class="border border-gray-300 p-2 text-right">Värde (SEK)</th>
                <th class="border border-gray-300 p-2 text-right">Förändring</th>
                <th class="border border-gray-300 p-2 text-right">Inköpspris</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="entry in report.entries"
                :key="entry.item_name"
                class="hover:bg-gray-50"
              >
                <td class="border border-gray-300 p-2 font-semibold">{{ entry.item_name }}</td>
                <td class="border border-gray-300 p-2">{{ formatAlcoholType(entry.alcohol_type) }}</td>
                <td class="border border-gray-300 p-2">{{ entry.supplier }}</td>
                <td class="border border-gray-300 p-2 text-right">{{ entry.volume_cl }}</td>
                <td class="border border-gray-300 p-2 text-right">
                  {{ entry.current_bottles }}
                </td>
                <td
                  class="border border-gray-300 p-2 text-right font-semibold"
                  :class="entry.bottle_change >= 0 ? 'text-green-600' : 'text-red-600'"
                >
                  {{ entry.bottle_change >= 0 ? '+' : '' }}{{ entry.bottle_change }}
                </td>
                <td class="border border-gray-300 p-2 text-right">
                  {{ formatCurrency(entry.current_total_value) }}
                </td>
                <td
                  class="border border-gray-300 p-2 text-right font-semibold"
                  :class="entry.value_change >= 0 ? 'text-green-600' : 'text-red-600'"
                >
                  {{ entry.value_change >= 0 ? '+' : '' }}{{ formatCurrency(entry.value_change) }}
                </td>
                <td class="border border-gray-300 p-2 text-right">
                  {{ formatCurrency(entry.current_purchase_price) }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <div v-else class="text-center text-gray-500 py-8">
      Ingen rapport tillgänglig. Klicka "Ladda rapport" för att generera en.
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ArrowDownTrayIcon } from '@heroicons/vue/16/solid'
import { getAlcoholReport } from '@/stores/alcoholData'
import type { AlcoholInventoryReport, AlcoholType } from '@/types'
import { useNotificationsStore } from '@/stores/notifications'

const notificationsStore = useNotificationsStore()
const report = ref<AlcoholInventoryReport | null>(null)

const formatAlcoholType = (type: AlcoholType): string => {
  const labels: Record<AlcoholType, string> = {
    cider: 'Cider',
    beer: 'Öl',
    spirits: 'Sprit',
    wine: 'Vin',
  }
  return labels[type] || type
}

const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleDateString('sv-SE', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

const formatCurrency = (value: number): string => {
  return value.toFixed(2)
}

const formatPercentage = (change: number, total: number): string => {
  if (total === 0) return '0'
  const percentage = (change / total) * 100
  return percentage >= 0 ? `+${percentage.toFixed(1)}` : `${percentage.toFixed(1)}`
}

const loadReport = async () => {
  try {
    report.value = await getAlcoholReport()
    notificationsStore.addNotification({
      id: Math.random(),
      title: 'Success',
      message: 'Rapport laddad',
      severity: 'info',
    })
  } catch (error) {
    notificationsStore.addNotification({
      id: Math.random(),
      title: 'Error',
      message: 'Misslyckades att ladda rapport',
      severity: 'error',
    })
  }
}
</script>
