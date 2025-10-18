<template>
  <div class="main">
    <PanelTemplate title="Leverantörer" :icon="ShoppingCartIcon" :button-left-icon="PlusIcon"
      :buttonLeftRestricted="!permsStore.hasWriteAccess()" @buttonLeft="addSupplier()">
      <DynamicTable :rows="rows" :columns="columns" settings>
        <template #row="input">
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <a v-if="input.row.link" class="hover:underline" :href="input.row.link" target="_blank">
              {{ input.row.name }}
            </a>
            <p v-else>{{ input.row.name }}</p>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <p>{{ input.row.group }}</p>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <p>{{ input.row.username }}</p>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <p>{{ input.row.password }}</p>
          </td>
          <td class="p-2 border-b border-(--zaiko-bg-2)">
            <p>{{ input.row.notes }}</p>
          </td>
        </template>
        <template #settings="input">
          <HamMenu :rows="settings.props.rows" @select="(action) => Settings(action, input.row.name)" />
        </template>
      </DynamicTable>
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import { ref, type FunctionalComponent } from 'vue';
import PanelTemplate from '@/components/PanelTemplate.vue';
import { BackspaceIcon, PencilSquareIcon, PlusIcon, ShoppingCartIcon } from '@heroicons/vue/24/outline';
import type { SupplierDeleteRequest, SupplierGetResponse } from '@/types';
import { usePopupStore } from '@/stores/popup';
import SupplierForm from '@/components/SupplierForm.vue';
import { deleteSupplier, getSuppliers } from '@/stores/supplierData';
import DynamicTable from '@/components/DynamicTable.vue';
import { usePermsStore } from '@/stores/permissions';
import HamMenu from '@/components/HamMenu.vue';
import DeleteForm from '@/components/DeleteForm.vue';

const popupStore = usePopupStore();
const permsStore = usePermsStore();

function addSupplierGhost(result?: any) {
  if (result) {
    console.log("Adding new supplier to list:", result);
    rows.value.push(result);
  }
}

const addSupplier = () => {
  popupStore.push({
    title: 'Lägg till leverantör',
    component: SupplierForm,
    icon: PlusIcon,
    cb: addSupplierGhost,
  })
}

const columns = {
  name: 'Namn',
  group: 'Grupp',
  username: 'Användarnamn',
  password: 'Lösenord',
  notes: 'Noteringar',
};

const rows = ref<SupplierGetResponse>([]);
getSuppliers().then((data) => {
  rows.value = data;
});

const settings = {
  props: {
    rows: { edit: 'redigera', delete: 'ta bort' },
  },
}

function removeSupplierGhost(result?: any) {
  if (result) {
    rows.value = rows.value.filter((supplier) => supplier.name !== result.name);
  }
}

function editSupplierGhost(result?: any) {
  if (result) {
    const index = rows.value.findIndex((supplier) => supplier.name === result.name);
    if (index !== -1) {
      rows.value[index] = result;
    }
  }
}

const Settings = (action: string, name: string) => {
  let title = "";
  let comp = null;
  let icon: FunctionalComponent = PlusIcon;
  let props = {};
  let cb = undefined;
  switch (action) {
    case 'edit':
      title = "Redigera " + name;
      comp = SupplierForm;
      props = {
        edit: true,
        editSupplier: rows.value.find((supplier) => supplier.name === name)
      };
      icon = PencilSquareIcon
      cb = editSupplierGhost
      break;
    case 'delete':
      comp = DeleteForm;
      title = "Är du säker?";
      props = {
        item: { name: name } as SupplierDeleteRequest,
        deleteFunc: deleteSupplier
      };
      icon = BackspaceIcon
      cb = removeSupplierGhost
      break;
    default:
      console.error("Unknown action:", action, name);
      break;
  }
  if (comp) {
    popupStore.push({
      title: title,
      component: comp,
      props: props,
      icon: icon,
      cb: cb,
    })
  }
}
</script>

<style scoped></style>
