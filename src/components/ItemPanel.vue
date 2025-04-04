<template>
  <div>
    <form v-on:submit.prevent="updateItem">
      <div class="item">
        <div class="itemHeader">
          <ArchiveBoxIcon class="buttonIcon" />
          <p>Produkt</p>
        </div>
        <input v-model="name" placeholder="Produkt" required minlength=1>
      </div>
      <div class="item">
        <div class="itemHeader">
          <HomeIcon class="buttonIcon" />
          <p>Plats</p>
        </div>
        <input v-model="location" placeholder="Plats" required minlength=1>
      </div>
      <fieldset>
        <div class="item">
          <div class="itemHeader">
            <Battery0Icon class="buttonIcon" />
            <p>Min</p>
          </div>
          <input type="number" v-model="min" placeholder="Min">
        </div>
        <div class="item">
          <div class="itemHeader">
            <Battery100Icon class="buttonIcon" />
            <p>Max</p>
          </div>
          <input type="number" v-model="max" placeholder="Max">
        </div>
        <div class="item">
          <div class="itemHeader">
            <Battery50Icon class="buttonIcon" />
            <p>Nuvarande</p>
          </div>
          <input type="number" v-model="current" placeholder="Nuvarande">
        </div>
      </fieldset>
      <div class="item">
        <div class="itemHeader">
          <ShoppingCartIcon class="buttonIcon" />
          <p>Leverantör</p>
        </div>
        <select class="input" v-model="supplier" placeholder="Leverantör">
          <option v-for="supplier in supplierStore.suppliers" :key="supplier.id" :value="supplier.id"
            :selected="item.supplier == supplier.id">{{ supplier.name }}</option>
        </select>
      </div>
      <div class="item">
        <div class="itemHeader">
          <LinkIcon class="buttonIcon" />
          <p>Länk</p>
        </div>
        <input type="url" v-model="link" placeholder="Länk">
      </div>
      <div class="submitEdit">
        <button type="submit">
          <DocumentCheckIcon class="buttonIcon" />
          <p>Spara</p>
        </button>
        <button class="delete" @click.prevent="Delete">
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
import { useSupplierStore } from '@/stores/suppliers';
import { useItemStore } from '@/stores/items';
import type { ItemUpdateRequest } from '@/types';


const { id } = defineProps<{
  id: number,
}>()
console.log(id)

const supplierStore = useSupplierStore();
const itemStore = useItemStore();
const item = await itemStore.getItem(id);

const name = ref<string>(item.name)
const location = ref<string>(item.location)
const min = ref<number | undefined>(item.min)
const max = ref<number | undefined>(item.max)
const current = ref<number>(item.current)
const supplier = ref<number | undefined>(item.supplier)
const link = ref<string | undefined>(item.link)

const emit = defineEmits(["submit", "delete"]);

const updateItem = async () => {
  const updatedItem: ItemUpdateRequest = {
    id: item.id,
    name: name.value,
    location: location.value,
    min: min.value,
    max: max.value,
    current: current.value,
    supplier: supplier.value,
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
  gap: 2rem;
  margin: 2rem auto;
}

p {
  margin: 0;
}

form {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  gap: 2rem;
  width: 100%;
}

fieldset {
  all: unset;
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 1rem;
  width: 100%;
}

fieldset .item input {
  max-width: 100px;
}

.submitEdit {
  display: flex;
  align-items: center;
  flex-direction: row-reverse;
  gap: 1rem;
}

select {
  all: unset;
  padding: 0.5rem;
  appearance: auto;
  font-size: 1rem;
  border: none;
  border-radius: 5px;
}

.delete {
  display: flex;
  align-items: center;
  flex-direction: row;
  gap: 0.5rem;
  font-size: 1.1rem;
  padding: 0.6rem;
  background-color: #B62E3D;
  color: #FAFAFA;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

button[type="submit"] {
  display: flex;
  align-items: center;
  flex-direction: row;
  gap: 0.5rem;
  font-size: 1.1rem;
  padding: 0.6rem;
  background-color: #2EB563;
  color: #FAFAFA;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

input[type="number"] {
  -moz-appearance: textfield;
  -webkit-appearance: textfield;
  appearance: textfield;
}

input {
  font-size: 0.9rem;
}

.buttonIcon {
  width: 1.5rem;
  height: 1.5rem;
}

.itemHeader {
  display: flex;
  align-items: center;
  gap: 4px;
}

.itemHeader svg {
  color: rgba(0, 0, 0, 0.33);
}

.item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 2px solid rgba(0, 0, 0, 0.33);
  padding: 8px 0;
  margin-bottom: 8px;
  max-width: 100%;
}

.item input {
  border: none;
  background-color: inherit;
  text-align: right;
  width: 100%;
}

@media (max-width: 700px) {
  .main-content {
    margin: 0;
    gap: 0.5rem;
  }

  fieldset {
    grid-template-columns: 1fr;
    gap: 0.7rem;
  }

  h1 {
    font-size: 2.55rem;
  }
}
</style>
