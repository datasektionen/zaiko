<template>
  <div class="">
    <div class="flex flex-col gap-4 justify-center">
      <PanelTemplate :title="item.name" :icon="ArchiveBoxIcon">
        <div class="flex flex-col md:flex-row gap-8 justify-center">
          <LongButton title="Ändra" :icon="PencilSquareIcon" @click="editName()" v-if="permsStore.hasWriteAccess()" />
          <LongButton title="Loggar" :icon="PencilSquareIcon" :link="'/logs/' + item.name" />
        </div>
      </PanelTemplate>
      <PanelTemplate title="Lager" :icon="InboxIcon" :button-left-icon="PlusIcon"
        :buttonLeftRestricted="!permsStore.hasWriteAccess()" @buttonLeft="addItem()"
        :button-right-icon="ArrowUpTrayIcon" :buttonRightRestricted="!permsStore.hasWriteAccess()"
        @buttonRight="moveItem()">
        <DynamicTable :rows="item.storage!" :columns="storageColumns" settings>
          <template #row="input">
            <td class="p-2 border-b border-(--zaiko-bg-2)">
              <RouterLink :to="'/storage/' + encodeURIComponent(input.row.storage)">
                <p class="hover:underline">{{ input.row.storage }}</p>
              </RouterLink>
            </td>
            <td class="p-2 border-b border-(--zaiko-bg-2)">
              <RouterLink
                :to="'/storage/' + encodeURIComponent(input.row.storage) + (input.row.container !== '' ? ('?container=' + encodeURIComponent(input.row.container)) : '')">
                <p class="hover:underline">{{ input.row.container || "Ingen" }}</p>
              </RouterLink>
            </td>
            <td class="p-2 border-b border-(--zaiko-bg-2)">
              <p>{{ input.row.min }}</p>
            </td>
            <td class="p-2 border-b border-(--zaiko-bg-2)">
              <p>{{ input.row.max }}</p>
            </td>
            <td class="p-2 border-b border-(--zaiko-bg-2)">
              <p>{{ input.row.amount }}</p>
            </td>
            <td class="p-2 border-b border-(--zaiko-bg-2)">
              <p>{{ stateEmoji(input.row.state) }}</p>
            </td>
          </template>
          <template #settings="input">
            <HamMenu :rows="settings.props.rows" v-if="permsStore.writeAccessToStorage(input.row.storage)"
              @select="(action) => Settings(action, input.row.storage, input.row.container)" />
          </template>
        </DynamicTable>
      </PanelTemplate>
      <PanelTemplate title="Leverantörer" :icon="ShoppingCartIcon" :button-left-icon="LinkIcon"
        :buttonLeftRestricted="!permsStore.hasWriteAccess()" @buttonLeft="linkItem()">
        <DynamicTable :rows="item.supplier!" :columns="supplierColumns" settings>
          <template #row="input">
            <td class="p-2 border-b border-(--zaiko-bg-2)">
              <RouterLink :to="'/supplier/' + encodeURIComponent(input.row.name)" class="flex items-center gap-1">
                <TrophyIcon class="w-5 h-5 text-(--zaiko-warning-color) inline-block mr-1" v-if="input.row.prfered" />
                <p class="hover:underline">{{ input.row.name }}</p>
              </RouterLink>
            </td>
            <td class="p-2 border-b border-(--zaiko-bg-2)">
              <a v-if="input.row.link" :href="input.row.link" target="_blank" class="hover:underline">
                <span class="w-5 aspect-square inline-block text-(--zaiko-link-color)">
                  <LinkIcon />
                </span>
              </a>
            </td>
          </template>
          <template #settings="input">
            <HamMenu :rows="settingsSupplier.props.rows"
              @select="(action) => SettingsSupplier(action, input.row.name)" />
          </template>
        </DynamicTable>
      </PanelTemplate>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, type FunctionalComponent } from 'vue';
import type { ItemAddRequest, ItemDeleteRequest, ItemGetResponse, ItemEditRequest, ItemUnlinkSupplierRequest } from '@/types';
import PanelTemplate from '@/components/PanelTemplate.vue';
import { ArchiveBoxIcon, ArrowUpTrayIcon, BackspaceIcon, InboxIcon, LinkIcon, PencilSquareIcon, PlusIcon, ShoppingCartIcon, TrophyIcon } from '@heroicons/vue/24/outline';
import DynamicTable from '@/components/DynamicTable.vue';
import { RouterLink, useRoute, useRouter } from 'vue-router';
import { changeItemName, deleteItem, getItemByName, supplierUnlinkItem } from '@/stores/itemData';
import { usePopupStore } from '@/stores/popup';
import ItemForm from '@/components/ItemForm.vue';
import LinkForm from '@/components/LinkForm.vue';
import LongButton from '@/components/LongButton.vue';
import DeleteForm from '@/components/DeleteForm.vue';
import HamMenu from '@/components/HamMenu.vue';
import NameForm from '@/components/NameForm.vue';
import { containerText } from '@/stores/inventoryData';
import { usePermsStore } from '@/stores/permissions';
import { stateEmoji } from '@/common';

const permsStore = usePermsStore();

const route = useRoute();
const item = ref<ItemGetResponse>({
  name: decodeURI(route.params.name as string),
  storage: [],
  supplier: [],
  avrage_consuption: 0,
  unit: 'st',
  inventory_interval: 'P0D',
});

const popupStore = usePopupStore();
const moveItem = () => {
  popupStore.push({
    title: 'Flytta produkt',
    component: ItemForm,
    icon: LinkIcon,
  })
}
const linkItem = () => {
  popupStore.push({
    title: 'Koppla leverantör',
    component: LinkForm,
    props: {
      name: item.value.name,
    },
    icon: LinkIcon,
    cb: addSupplierGhost,
  })
}

const router = useRouter();
const editName = () => {
  popupStore.push({
    title: 'Ändra namn på ' + item.value.name,
    component: NameForm,
    icon: PencilSquareIcon,
    props: {
      item: {
        name: item.value.name,
        new_name: item.value.name,
        unit: item.value.unit,
        inventory_interval: item.value.inventory_interval,
      } as ItemEditRequest,
      editFunc: changeItemName,
    },
    cb: (result: any) => {
      router.push('/item/' + encodeURIComponent(result.new_name)).then(() => {
        getItemByName(result.new_name).then((data) => {
          item.value = data;
        });
      });
    },
  })
}

const addItem = () => {
  popupStore.push({
    title: 'Lägg till ' + item.value.name + ' i lager',
    component: ItemForm,
    icon: PlusIcon,
    props: {
      edit: false,
      editItem: {
        name: item.value.name,
        unit: item.value.unit || undefined,
      }
    },
    cb: addItemGhost,
  })
}

function addItemGhost(result?: any) {
  if (result) {
    getItemByName(item.value.name).then((data) => {
      item.value = data;
    });
  }
}

const Settings = (action: string, storage: string, container: string) => {
  let title = "";
  let comp = null;
  let icon: FunctionalComponent = PlusIcon;
  let props = {};
  let cb = undefined;
  switch (action) {
    case 'edit':
      comp = ItemForm;
      const editItem = item.value.storage?.find((s) => s.storage === storage && s.container === container);
      props = {
        edit: true,
        editItem: {
          name: item.value.name,
          unit: item.value.unit || undefined,
          storage: editItem?.storage || storage,
          container: editItem?.container || container,
          min: editItem?.min || undefined,
          max: editItem?.max || undefined,
          amount: editItem?.amount || 0,
        } as ItemAddRequest
      }
      icon = PencilSquareIcon
      title = "Redigera " + item.value.name + " i " + storage + containerText(container);
      cb = editItemGhost
      break;
    case 'delete':
      comp = DeleteForm;
      title = "Är du säker?";
      props = {
        item: { name: item.value.name, storage: storage, container: container } as ItemDeleteRequest,
        deleteFunc: deleteItem
      };
      icon = BackspaceIcon
      cb = removeItemGhost
      break;
    default:
      console.error("Unknown action:", action, item);
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

const SettingsSupplier = (action: string, name: string) => {
  let title = "";
  let comp = null;
  let icon: FunctionalComponent = PlusIcon;
  let props = {};
  let cb = undefined;
  switch (action) {
    case 'edit':
      // comp = StorageForm;
      props = {}; // add item
      icon = PencilSquareIcon
      cb = undefined
      break;
    case 'unsupply':
      comp = DeleteForm;
      title = "Är du säker?";
      props = {
        item: { name: item.value.name, supplier: name } as ItemUnlinkSupplierRequest,
        deleteFunc: supplierUnlinkItem
      };
      icon = BackspaceIcon
      cb = removeSupplierGhost
      break;
    default:
      console.error("Unknown action:", action, item);
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

function removeItemGhost(result?: any) {
  if (result) {
    item.value.storage = item.value.storage?.filter(s => !(s.storage === result.storage && s.container === result.container));
  }
}

function removeSupplierGhost(result?: any) {
  if (result) {
    item.value.supplier = item.value.supplier?.filter(s => s.name !== result.supplier);
  }
}

function addSupplierGhost(result?: any) {
  if (result) {
    getItemByName(item.value.name).then((data) => {
      item.value = data;
    });
  }
}

function editItemGhost(result?: any) {
  if (result) {
    getItemByName(item.value.name).then((data) => {
      item.value = data;
    });
  }
}

const settings = {
  props: {
    rows: { edit: 'redigera', delete: 'ta bort' },
  },
}
const settingsSupplier = {
  props: {
    rows: { edit: 'Redigera', unsupply: 'Koppla bort' },
  },
}

const storageColumns = {
  storage: 'Lager',
  container: 'Låda',
  min: 'Min',
  max: 'Max',
  amount: 'Mängd',
  state: 'Status',
};

const supplierColumns = {
  name: 'Namn',
  link: 'Länk',
};


getItemByName(item.value.name).then((data) => {
  console.log('item:', data);
  item.value = data;
  item.value.supplier = item.value.supplier?.sort((a, b) => a.prfered ? -1 : b.prfered ? 1 : 0 || a.name.localeCompare(b.name));
});

</script>

<style scoped></style>
