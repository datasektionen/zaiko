<template>
  <div>
    <form v-on:submit.prevent="addItem">
      <div class="groupHeader">
        <InputText name="Produkt" :icon="ArchiveBoxIcon" v-model="name" club-perm="rw" required />
        <InputText name="Plats" :icon="HomeIcon" v-model="location" club-perm="rw" />
      </div>
      <fieldset>
        <InputNumber name="Min" :icon="Battery0Icon" v-model="min" club-perm="rw" />
        <InputNumber name="Max" :icon="Battery100Icon" v-model="max" club-perm="rw" />
        <InputNumber name="Nuvarande" :icon="Battery50Icon" v-model="current" club-perm="rw" required />
      </fieldset>
      <InputSelect name="Leverantör" :icon="ShoppingCartIcon" v-model="supplier" club-perm="rw"
        :items="supplierStore.suppliers" />
      <InputText name="Länk" :icon="LinkIcon" v-model="link" club-perm="rw" />
      <div class="submit">
        <button type="submit" class="goodButton">
          <DocumentCheckIcon class="buttonIcon" />
          <p>Lägg till</p>
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { ItemAddRequest } from '@/types';
import { ArchiveBoxIcon, ShoppingCartIcon, HomeIcon, LinkIcon, DocumentCheckIcon, Battery0Icon, Battery100Icon, Battery50Icon } from '@heroicons/vue/16/solid';
import { useSupplierStore } from '@/stores/suppliers';
import { useItemStore } from '@/stores/items';
import InputText from '@/components/InputText.vue';
import InputNumber from '@/components/InputNumber.vue';
import InputSelect from '@/components/InputSelect.vue';

const supplierStore = useSupplierStore();
const itemStore = useItemStore();

const name = ref<string>("")
const location = ref<string>("")
const min = ref<number>()
const max = ref<number>()
const current = ref<number>(0)
const supplier = ref<number>(-1)
const link = ref<string>()

const emit = defineEmits(["submit"]);

const addItem = async () => {
  const item: ItemAddRequest = {
    name: name.value,
    location: location.value,
    min: min.value,
    max: max.value,
    current: current.value,
    supplier: supplier.value == -1 ? undefined : supplier.value,
    link: link.value
  };
  await itemStore.addItem(item);
  emit('submit')
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
  grid-template-columns: 1fr 1fr 1fr;
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
