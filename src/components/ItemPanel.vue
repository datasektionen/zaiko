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
          <input v-model="current" placeholder="Nuvarande">
        </div>
      </fieldset>
      <div class="item">
        <div class="itemHeader">
          <ShoppingCartIcon class="buttonIcon" />
          <p>Leverantör</p>
        </div>
        <select class="input" v-model="supplier" placeholder="Leverantör">
          <option v-for="supplier in suppliers" :key="supplier.id" :value="supplier.id"
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
import type { ItemUpdateRequest, ItemGetResponse, SupplierListGetResponse, Notification } from '@/types';
import { ArchiveBoxIcon, ShoppingCartIcon, HomeIcon, LinkIcon, BackspaceIcon, DocumentCheckIcon, Battery0Icon, Battery100Icon, Battery50Icon } from '@heroicons/vue/16/solid';
import { useClubsStore } from '@/stores/clubs';
import { useNotificationsStore } from '@/stores/notifications';
const HOST = import.meta.env.VITE_HOST;

const { item } = defineProps<{
  item: ItemGetResponse,
}>()


const notificationsStore = useNotificationsStore();
const clubStore = useClubsStore();

const suppliers = ref<Array<SupplierListGetResponse>>([])

const GetSuppliers = () => {
  if (clubStore.getClub() == "Nämnd") {
    const noti: Notification = {
      id: Date.now(),
      title: "Error",
      message: "Nämnd har ingen leverantör",
      severity: "error",
    }
    notificationsStore.add(noti);
    return;
  };
  const url: string = HOST + "/api/" + clubStore.displayClub() + "/suppliers";
  fetch(url, {
    method: "GET",
  }).then((r) => r.json())
    .then((json) => {
      suppliers.value = json
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
}
GetSuppliers();

const name = ref<string>(item.name)
const location = ref<string>(item.location)
const min = ref<number | undefined>(item.min)
const max = ref<number | undefined>(item.max)
const current = ref<number>(item.current)
const supplier = ref<number | undefined>(item.supplier)
const link = ref<string | undefined>(item.link)

const emit = defineEmits(["submit", "delete"]);

const updateItem = async () => {
  const res: ItemUpdateRequest = {
    id: item.id,
    name: name.value,
    location: location.value,
    min: min.value,
    max: max.value,
    current: current.value,
    supplier: supplier.value,
    link: link.value,
  }
  const url: string = HOST + "/api/" + clubStore.displayClub();
  await fetch(url + "/item", {
    method: "PATCH",
    body: JSON.stringify(res),
  })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Updaterad",
          message: "Produkten har uppdaterats",
          severity: "info",
        }
        notificationsStore.add(noti);
      } else {
        res.text().then((text) => text).then((text) => {
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
  emit("submit")
}

const Delete = async () => {
  const url: string = HOST + "/api/" + clubStore.displayClub();
  const query = new URLSearchParams({ id: item.id.toString() }).toString();
  await fetch(url + "/item?" + query, {
    method: "DELETE",
  })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Raderad",
          message: "Produkten har tagits bort",
          severity: "info",
        }
        notificationsStore.add(noti);
      } else {
        res.text().then((text) => text).then((text) => {
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
