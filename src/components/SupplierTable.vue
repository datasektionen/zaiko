<template>
  <div>
    <table v-if="supplierStore.suppliers.length > 0" class="table">
      <thead>
        <tr>
          <th scope="col">
            <span>
              <ShoppingCartIcon class="tableIcon" />
              <p v-if="!isMobile">Namn</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <UserCircleIcon class="tableIcon" />
              <p v-if="!isMobile">Användarnamn</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <LockClosedIcon class="tableIcon" />
              <p v-if="!isMobile">Lösenord</p>
            </span>
          </th>
          <th scope="col">
            <span>
              <DocumentTextIcon class="tableIcon" />
              <p v-if="!isMobile">Anteckningar</p>
            </span>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in supplierStore.suppliers" :key="item.id" @click="emit('select', item.id)">
          <td scope="row">
            <a :href="item.link" target="_blank" @click.stop="" v-if="item.link">{{ item.name }}</a>
            <p v-else>{{ item.name }}</p>
          </td>
          <td>{{ item.username }}</td>
          <td>{{ item.password }}</td>
          <td>{{ item.notes }}</td>
        </tr>
      </tbody>
    </table>
    <div v-else>
      <EmptyTable :compact="isMobile.value" text="Inga leverantörer" :icon="ReceiptPercentIcon" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { LockClosedIcon, ShoppingCartIcon, UserCircleIcon, DocumentTextIcon } from '@heroicons/vue/16/solid'
import { ReceiptPercentIcon } from '@heroicons/vue/24/outline';
import { useMediaQuery } from '@vueuse/core/index.cjs';
import { useSupplierStore } from '@/stores/suppliers';
import EmptyTable from './EmptyTable.vue';

const supplierStore = useSupplierStore();

const emit = defineEmits(['select'])
const isMobile = useMediaQuery('(max-width: 768px)');

if (supplierStore.suppliers.length == 0) {
  await supplierStore.fetchSuppliers();
}

</script>

<style scoped>
</style>
