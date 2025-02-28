<template>
  <div class="main-content">
    <form v-on:submit.prevent="updateSupplier">
      <div class="item">
        <div class="itemHeader">
          <ShoppingCartIcon class="buttonIcon" />
          <p>Namn</p>
        </div>
        <input v-model="name" placeholder="Namn" required minlength=1>
      </div>
      <div class="item">
        <div class="itemHeader">
          <UserCircleIcon class="buttonIcon" />
          <p>Användarnamn</p>
        </div>
        <input v-model="username" placeholder="Användarnamn">
      </div>
      <div class="item">
        <div class="itemHeader">
          <LockClosedIcon class="buttonIcon" />
          <p>Lösenord</p>
        </div>
        <input v-model="password" placeholder="Lösenord">
      </div>
      <div class="item">
        <div class="itemHeader">
          <LinkIcon class="buttonIcon" />
          <p>Länk</p>
        </div>
        <input type="url" v-model="link" placeholder="Länk">
      </div>
      <div class="item itemArea">
        <div class="itemHeader">
          <DocumentTextIcon class="buttonIcon" />
          <p>Anteckningar</p>
        </div>
        <textarea v-model="note" placeholder="Anteckningar"></textarea>
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
import { useClubsStore } from '@/stores/clubs';
import { useNotificationsStore } from '@/stores/notifications';
import type { SupplierGetResponse, SupplierUpdateRequest, Notification } from '@/types';
import { BackspaceIcon, ShoppingCartIcon, LinkIcon, UserCircleIcon, LockClosedIcon, DocumentTextIcon, DocumentCheckIcon } from '@heroicons/vue/16/solid'
import { ref, defineProps } from 'vue';
const HOST = import.meta.env.VITE_HOST;

const emit = defineEmits(["submit", "delete"]);
const { item } = defineProps<{
  item: SupplierGetResponse,
}>()

const notificationsStore = useNotificationsStore();
const clubStore = useClubsStore();

const name = ref(item.name)
const username = ref(item.username)
const password = ref(item.password)
const link = ref(item.link)
const note = ref(item.notes)

const updateSupplier = async () => {
  if (!clubStore.checkClub()) return;
  const url: string = HOST + "/api/" + clubStore.getClub();
  const supplier: SupplierUpdateRequest = {
    id: item.id,
    name: name.value,
    username: username.value,
    password: password.value,
    link: link.value,
    notes: note.value,
  }
  await fetch(url + "/supplier", {
    method: "PATCH",
    body: JSON.stringify(supplier),
  })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Updaterad",
          message: "Leverantören har uppdaterats",
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
  emit("submit")
}

const Delete = async () => {
  if (!clubStore.checkClub()) return;
  const url: string = HOST + "/api/" + clubStore.getClub();
  const query = new URLSearchParams({ id: item.id.toString() }).toString();

  await fetch(
    url + "/supplier?" + query,
    {
      method: "DELETE",
    })
    .then((res) => {
      if (res.ok) {
        const noti: Notification = {
          id: Date.now(),
          title: "Borttagen",
          message: "Leverantören har tagits bort",
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
  gap: 1.2rem;
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

.itemArea {
  flex-direction: column;
  align-items: stretch;
  justify-content: flex-start;
}

.itemArea textarea {
  border: none;
  background-color: inherit;
  width: 100%;
  min-height: 120px;
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
