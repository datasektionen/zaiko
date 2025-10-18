<template>
  <div>
    <form v-on:submit.prevent="linkItem()">
      <InputSelect name="Leverantör*" :icon="ShoppingCartIcon" v-model="supplier" :items="suppliers" required>
        <template #row="item">
          <option :key="item.row.name" :value="item.row.name">
            {{ item.row.name }}
          </option>
        </template>
      </InputSelect>
      <InputText name="Länk" :icon="LinkIcon" v-model="link" />
      <InputCheckbox name="Föredragen leverantör" :icon="TrophyIcon" v-model="prefered" />
      <div class="submit justify-end">
        <button type="submit"
          class="flex items-center gap-2 p-2 bg-(--zaiko-main-color) text-(--zaiko-text-hc) rounded-md">
          <DocumentCheckIcon class="w-8 aspect-square" />
          <p>Länka</p>
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ShoppingCartIcon, LinkIcon, DocumentCheckIcon } from '@heroicons/vue/16/solid';
import InputText from '@/components/InputText.vue';
import InputSelect from '@/components/InputSelect.vue';
import { type ItemLinkSupplierRequest, type SupplierGetResponse } from '@/types';
import { supplierLinkItem } from '@/stores/itemData';
import { usePopupStore } from '@/stores/popup';
import InputCheckbox from '@/components/InputCheckbox.vue';
import { getSuppliers } from '@/stores/supplierData';
import { TrophyIcon } from '@heroicons/vue/24/outline';

const props = defineProps<{
  name: string
}>()

const link = ref<string>()
const supplier = ref<string>("")
const prefered = ref<boolean>(false)

const suppliers = ref<SupplierGetResponse>([])
getSuppliers().then((data) => {
  suppliers.value = data
})

const linkItem = () => {
  const payload: ItemLinkSupplierRequest = {
    name: props.name,
    link: link.value,
    supplier: supplier.value,
    prefered: prefered.value,
  }
  supplierLinkItem(payload).then(() => {
    const popupStore = usePopupStore();
    popupStore.callCurrent();
    popupStore.pop();
  }).catch((err) => {
    console.error("Error creating item:", err);
  })
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
