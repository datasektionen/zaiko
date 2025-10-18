<template>
  <div class="main-content">
    <form v-on:submit.prevent="edit ? EditSupplier() : addSupplier()">
      <InputSelect name="Grupp*" :icon="UserGroupIcon" v-model="group" :items="groups" required>
        <template #row="item">
          <option :key="item.row" :value="item.row">
            {{ item.row }}
          </option>
        </template>
      </InputSelect>
      <InputText v-model="name" name="Namn*" :icon="ShoppingCartIcon" required />
      <fieldset>
        <InputText v-model="username" name="Användarnamn" :icon="UserCircleIcon" />
        <InputText v-model="password" name="Lösenord" :icon="LockClosedIcon" type="password" />
      </fieldset>
      <InputText v-model="link" name="Länk" :icon="LinkIcon" :maxlength=10000 />
      <InputTextArea v-model="note" name="Anteckningar" :icon="DocumentTextIcon" />
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
import type { SupplierAddRequest } from '@/types';
import { ShoppingCartIcon, LinkIcon, UserCircleIcon, LockClosedIcon, DocumentTextIcon, DocumentCheckIcon } from '@heroicons/vue/16/solid'
import { ref } from 'vue';
import InputText from '@/components/InputText.vue';
import InputTextArea from '@/components/InputTextArea.vue';
import InputNumber from '@/components/InputNumber.vue';
import InputSelect from '@/components/InputSelect.vue';
import { UserGroupIcon } from '@heroicons/vue/24/outline';
import { createSupplier, updateSupplier } from '@/stores/supplierData';
import { usePopupStore } from '@/stores/popup';
import { usePermsStore } from '@/stores/permissions';

const props = defineProps<{
  edit?: boolean,
  editSupplier?: SupplierAddRequest,
}>();

const permsStore = usePermsStore();
const groups = ref<Array<string>>(permsStore.perms?.groups || []);

const name = ref(props.editSupplier?.name || "")
const group = ref<string>(props.editSupplier?.group || "")
const username = ref<string | undefined>(props.editSupplier?.username || undefined)
const password = ref<string | undefined>(props.editSupplier?.password || undefined)
const link = ref<string | undefined>(props.editSupplier?.link || "")
const note = ref<string | undefined>(props.editSupplier?.notes || "")

const addSupplier = async () => {
  const supplier: SupplierAddRequest = {
    group: group.value,
    name: name.value,
    username: username.value,
    password: password.value,
    link: link.value,
    notes: note.value
  };
  createSupplier(supplier).then(() => {
    const popupStore = usePopupStore();
    popupStore.callCurrent(supplier);
    popupStore.pop();
  }).catch((error) => {
    console.error("Failed to create supplier:", error);
  });
}

const EditSupplier = async () => {
  const supplier: SupplierAddRequest = {
    group: group.value,
    name: name.value,
    username: username.value,
    password: password.value,
    link: link.value,
    notes: note.value
  };
  console.log("Updating supplier with data:", supplier);
  updateSupplier(supplier).then(() => {
    const popupStore = usePopupStore();
    popupStore.callCurrent(supplier);
    popupStore.pop();
  }).catch((error) => {
    console.error("Failed to update supplier:", error);
  });
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
