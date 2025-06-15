<template>
  <div class="main-content">
    <form v-on:submit.prevent="updateSupplier">
      <InputText v-model="name" name="Namn" :icon="ShoppingCartIcon" :clubPerm="clubPerm" />
      <ImputText v-model="username" name="Användarnamn" :icon="UserCircleIcon" :clubPerm="clubPerm" />
      <ImputText v-model="password" name="Lösenord" :icon="LockClosedIcon" :clubPerm="clubPerm" />
      <InputText v-model="link" name="Länk" :icon="LinkIcon" :clubPerm="clubPerm" />
      <InputArea v-model="note" name="Anteckningar" :icon="DocumentTextIcon" :clubPerm="clubPerm" />
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
import { useClubsStore } from '@/stores/clubs';
import { useSupplierStore } from '@/stores/suppliers';
import { BackspaceIcon, ShoppingCartIcon, LinkIcon, UserCircleIcon, LockClosedIcon, DocumentTextIcon, DocumentCheckIcon } from '@heroicons/vue/16/solid'
import { ref } from 'vue';
import InputText from '@/components/InputText.vue';
import InputNumber from '@/components/InputNumber.vue';
import InputSelect from '@/components/InputSelect.vue';
import InputArea from '@/components/InputTextArea.vue';

const emit = defineEmits(["submit", "delete"]);
const { id } = defineProps<{
  id: number,
}>()

const clubStore = useClubsStore();
const clubPerm = (await clubStore.getClub()).active.permission;

const supplierStore = useSupplierStore();
const item = await supplierStore.getSupplier(id);

const name = ref<string>(item.name)
const username = ref<string | undefined>(item.username)
const password = ref<string | undefined>(item.password)
const link = ref<string | undefined>(item.link)
const note = ref<string | undefined>(item.notes)

const updateSupplier = async () => {
  const updatedSupplier = {
    id: item.id,
    name: name.value,
    username: username.value,
    password: password.value,
    link: link.value,
    notes: note.value
  };
  await supplierStore.updateSupplier(updatedSupplier);
  emit("submit")
}

const Delete = async () => {
  await supplierStore.deleteSupplier(id);
  emit("delete")
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

.submitEdit {
  display: flex;
  align-items: center;
  flex-direction: row-reverse;
  gap: 1rem;
}

.delete {
  background-color: var(--zaiko-bad-color);
}

.submit {
  display: flex;
  align-items: flex-end;
  flex-direction: column;
}

@media (max-width: 700px) {
  .main-content {
    margin: 0;
  }

  h1 {
    font-size: 2.55rem;
  }
}
</style>
