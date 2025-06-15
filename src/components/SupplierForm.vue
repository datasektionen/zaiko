<template>
  <div class="main-content">
    <form v-on:submit.prevent="addSupplier">
      <InputText v-model="name" name="Namn" :icon="ShoppingCartIcon" required club-perm="rw" />
      <InputText v-model="username" name="Användarnamn" :icon="UserCircleIcon" club-perm="rw" />
      <InputText v-model="password" name="Lösenord" :icon="LockClosedIcon" type="password" club-perm="rw" />
      <InputText v-model="link" name="Länk" :icon="LinkIcon" club-perm="rw" />
      <InputTextArea v-model="note" name="Anteckningar" :icon="DocumentTextIcon" club-perm="rw" />
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
import { useSupplierStore } from '@/stores/suppliers';
import type { SupplierAddRequest } from '@/types';
import { ShoppingCartIcon, LinkIcon, UserCircleIcon, LockClosedIcon, DocumentTextIcon, DocumentCheckIcon } from '@heroicons/vue/16/solid'
import { ref } from 'vue';
import InputText from '@/components/InputText.vue';
import InputTextArea from '@/components/InputTextArea.vue';
import InputNumber from '@/components/InputNumber.vue';

const supplierStore = useSupplierStore();


const emit = defineEmits(["submit"]);

const name = ref("")
const username = ref<string>()
const password = ref<string>()
const link = ref<string>()
const note = ref<string>()

const addSupplier = async () => {
  const supplier: SupplierAddRequest = {
    name: name.value,
    username: username.value,
    password: password.value,
    link: link.value,
    notes: note.value
  };
  await supplierStore.addSupplier(supplier);
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
  gap: 0.25rem;
  width: 100%;
}

.submit {
  display: flex;
  align-items: flex-end;
  flex-direction: column;
}

@media (max-width: 700px) {
  .main-content {
    margin: 0;
    gap: 0.5rem;
  }

  h1 {
    font-size: 2.55rem;
  }
}
</style>
