<template>
  <div>
    <form v-on:submit.prevent="addItem">
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
          <input type="number" v-model="current" placeholder="Nuvarande" required>
        </div>
      </fieldset>
      <div class="item">
        <div class="itemHeader">
          <ShoppingCartIcon class="buttonIcon" />
          <p>Leverantör</p>
        </div>
        <select class="input" v-model="supplier" placeholder="Leverantör">
          <option value="-1" selected>Leverantör</option>
          <option v-for="supplier in suppliers" :key="supplier.id" :value="supplier.id">{{ supplier.name }}</option>
        </select>
      </div>
      <div class="item">
        <div class="itemHeader">
          <LinkIcon class="buttonIcon" />
          <p>Länk</p>
        </div>
        <input type="url" v-model="link" placeholder="Länk">
      </div>
      <div class="submit">
        <button type="submit">
          <DocumentCheckIcon class="buttonIcon" />
          <p>Lägg till</p>
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { ItemAddRequest, SupplierListGetResponse, Notification } from '@/types';
import { ArchiveBoxIcon, ShoppingCartIcon, HomeIcon, LinkIcon, DocumentCheckIcon, Battery0Icon, Battery100Icon, Battery50Icon } from '@heroicons/vue/16/solid';
import { useClubsStore } from '@/stores/clubs';
import { useNotificationsStore } from '@/stores/notifications';
const HOST = import.meta.env.VITE_HOST;

const notificationsStore = useNotificationsStore();
const clubStore = useClubsStore();

const suppliers = ref<Array<SupplierListGetResponse>>([])

const GetSuppliers = async () => {
  if (clubStore.getClub() == "Nämnd") {
    const noti: Notification = {
      id: Date.now(),
      title: "Warning",
      message: "Nämnden har inga leverantörer",
      severity: "warning",
    }
    notificationsStore.add(noti);
    return;
  };
  const url: string = HOST + "/api/" + clubStore.displayClub() + "/suppliers";
  suppliers.value = await fetch(url, {
    method: "GET",
  }).then((r) => r.json())
    .catch((error) => {
      const noti: Notification = {
        id: Date.now(),
        title: "Error",
        message: error.toString(),
        severity: "error",
      }
      notificationsStore.add(noti);
    })
}
GetSuppliers();

const name = ref<string>("")
const location = ref<string>("")
const min = ref<number>()
const max = ref<number>()
const current = ref<number>(0)
const supplier = ref<number>(-1)
const link = ref<string>()

const emit = defineEmits(["submit"]);

const addItem = async () => {
  const supplierId = supplier.value === -1 ? undefined : supplier.value;
  const res: ItemAddRequest = {
    name: name.value,
    location: location.value,
    min: min.value,
    max: max.value,
    current: current.value,
    supplier: supplierId,
    link: link.value,
  }
  if (clubStore.getClub() == "Nämnd") {
    const noti: Notification = {
      id: Date.now(),
      title: "Error",
      message: "Ingen nämnd vald",
      severity: "error",
    }
    notificationsStore.add(noti);
    return;
  };
  const url: string = HOST + "/api/" + clubStore.displayClub() + "/item";
  await fetch(url, {
    method: "POST",
    body: JSON.stringify(res),
  })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Success",
          message: "Produkten lades till",
          severity: "info",
        }
        notificationsStore.add(noti);
      } else {
        res.text().then((text) => {
          const noti: Notification = {
            id: Date.now(),
            title: "Error",
            message: text,
            severity: "error",
          }
          notificationsStore.add(noti);
        })
      }
    })
    .catch((error) => {
      const noti: Notification = {
        id: Date.now(),
        title: "Error",
        message: error.toString(),
        severity: "error",
      }
      notificationsStore.add(noti);
    })
  emit('submit')
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

select {
  all: unset;
  padding: 0.5rem;
  appearance: auto;
  font-size: 1rem;
  border: none;
  border-radius: 5px;
}

.submit {
  display: flex;
  align-items: center;
  flex-direction: row;
  gap: 1rem;
}

button[type="submit"] {
  display: flex;
  align-items: center;
  flex-direction: row;
  gap: 0.5rem;
  font-size: 1.1rem;
  padding: 0.6rem;
  background-color: #2EB563;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

input[type="number"] {
  -moz-appearance: textfield;
  -webkit-appearance: textfield;
}

input::placeholder {
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

.submit {
  display: flex;
  align-items: flex-end;
  flex-direction: column;
}

@media (max-width: 700px) {
  .main-content {
    margin: 0;
    gap: 0.5rem;
    max-width: 90vw;
  }

  fieldset {
    grid-template-columns: 1fr;
    gap: 0.7rem;
  }
}
</style>
