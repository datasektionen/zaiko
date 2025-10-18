<template>
  <div>
    <form v-on:submit.prevent="updateItem">
      <div class="groupHeader">
        <InputText name="Produkt" :icon="ArchiveBoxIcon" v-model="name" :clubPerm="clubPerm" required />
        <InputText name="Plats" :icon="HomeIcon" v-model="location" :clubPerm="clubPerm" />
      </div>
      <fieldset>
        <InputNumber name="Min" :icon="Battery0Icon" v-model="min" :clubPerm="clubPerm" />
        <InputNumber name="Max" :icon="Battery100Icon" v-model="max" :clubPerm="clubPerm" />
        <InputNumber name="Nuvarande" :icon="Battery50Icon" v-model="current" :clubPerm="clubPerm" required />
      </fieldset>
      <InputSelect name="Leverantör" :icon="ShoppingCartIcon" v-model="supplier" :clubPerm="clubPerm" :items="supplierStore.suppliers" />
      <InputText name="Länk" :icon="LinkIcon" v-model="link" :clubPerm="clubPerm" />
      <div class="submitEdit" v-if="clubPerm == 'rw'">
        <button type="submit" class="goodButton">
          <DocumentCheckIcon class="buttonIcon" />
          <p>Spara</p>
        </button>
        <button class="delete goodButton" @click.prevent="Delete">
          <BackspaceIcon class="buttonIcon" />
          <p>Radera</p>
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps } from 'vue'
import { ArchiveBoxIcon, ShoppingCartIcon, HomeIcon, LinkIcon, BackspaceIcon, DocumentCheckIcon, Battery0Icon, Battery100Icon, Battery50Icon } from '@heroicons/vue/16/solid';
// import type { ItemUpdateRequest } from '@/types';
import InputText from '@/components/InputText.vue';
import InputNumber from '@/components/InputNumber.vue';
import InputSelect from '@/components/InputSelect.vue';

const emit = defineEmits(["submit", "delete"]);

const updateItem = async () => {
  const updatedItem: ItemUpdateRequest = {
    id: item.id,
    name: name.value,
    location: location.value,
    min: min.value,
    max: max.value,
    current: current.value,
    supplier: supplier.value == "Ingen" ? undefined : await supplierStore.getSupplierId(supplier.value),
    link: link.value
  };
  await itemStore.updateItem(updatedItem);

  emit("submit")
}

const Delete = async () => {
  await itemStore.deleteItem(item.id);
  emit("delete")
}

</script>

<style scoped>
.main-content {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: flex-start;
  margin: 2rem auto;
}

form {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  gap: 1rem;
  width: 100%;
  color: var(--zaiko-text);
}

fieldset {
  all: unset;
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 1rem;
  width: 100%;
}

.groupHeader {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.submitEdit {
  display: flex;
  align-items: center;
  flex-direction: row-reverse;
  gap: 1rem;
}

.delete {
  background-color: var(--zaiko-bad-color);
}

@media (max-width: 700px) {
  .main-content {
    margin: 0;
    gap: 0.25rem;
  }

  form {
    gap: 0.25rem;
  }

  fieldset {
    grid-template-columns: 1fr;
    gap: 0.25rem;
  }

  h1 {
    font-size: 2.55rem;
  }
}
</style>
