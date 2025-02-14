<template>
  <div class="main">
    <div class="left-panel">
      <div class="top-bar">
        <button class="teal darken-1" @click="openModal = true">Lägg till</button>
        <div class="top-bar filter-bar">
          <p>Filter</p>
          <input type="text">
        </div>
      </div>
      <div class="header">
        <p>Produkt</p>
        <p>Plats</p>
        <p>Mängd</p>
        <p>Status</p>
      </div>
      <div class="items">
        <FrontPageItem :item="item" v-for="item in items" :key="item.id" />
      </div>
    </div>
    <div class="left-panel" id="bristPanel">
      <div class="top-bar">
        <h2 class="brist">Brist</h2>
        <div class="top-bar filter-bar">
          <p>Filter</p>
          <input type="text">
        </div>
      </div>
      <div class="header">
        <p>Produkt</p>
        <p>Leverantör</p>
        <p>Mängd</p>
        <p>Att köpa</p>
      </div>
      <div class="items">
        <FrontPageShortageItem :item="item" v-for="item in shortage" :key="item.name" />
      </div>
    </div>
    <PopupModal :modal="openModal" @exit="openModal = false">
      <AddForm @done="DoneModal()" />
    </PopupModal>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import FrontPageItem from '@/components/FrontPageItem.vue'
import PopupModal from '@/components/PopupModal.vue'
import AddForm from '@/components/AddForm.vue'
import FrontPageShortageItem from '@/components/FrontPageShortageItem.vue';
import type { ItemGetResponse, StockGetResponse } from '@/types';
import { useNotificationsStore } from '@/stores/notifications';
import type { Notification } from '@/types';
import { useClubsStore } from '@/stores/clubs';

const HOST = import.meta.env.VITE_HOST;

const clubStore = useClubsStore();

const UpdateMethone = () => {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  window.methone_conf.update({
    login_href: "/?club",
  })
}

window.onload = UpdateMethone;

const items = ref<Array<ItemGetResponse>>();
const shortage = ref<Array<StockGetResponse>>()

const openModal = ref<boolean>(false)

const notificationsStore = useNotificationsStore();

const DoneModal = () => {
  openModal.value = false;
  GetData();
}

const GetData = () => {
  const url: string = HOST + "/api/" + clubStore.getClub();
  fetch(url + "/item", {
    method: "GET",
  })
    .then((res) => res.json())
    .then((json) => items.value = json)
    .catch((error) => {
        const noti: Notification = {
          id: Date.now(),
          title: "Error",
          message: error.toString(),
          severity: "error",
        }
        notificationsStore.add(noti);
    })

  fetch(url + "/stock")
    .then((res) => res.json())
    .then((json: Array<StockGetResponse>) => shortage.value = json)
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
GetData();
</script>

<style scoped>
div {
  border-radius: 2px;
}

.main {
  display: grid;
  grid-template-columns: 50% 50%;
  gap: 1rem;
  padding: 4rem;
}

.left-panel {
  width: 480px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.header {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  padding: 0.5rem;
  text-align: center;
  font-weight: bold;
}

.items {
  display: flex;
  align-items: stretch;
  flex-direction: column;
}

.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.filter-bar {
  background-color: rgba(224, 242, 241, 1);
  display: flex;
  gap: 1rem;
  padding: 10px 20px;
  margin: 0;
  align-items: center;
  justify-content: space-between;
}

.filter-bar p {
  margin: 0;
  text-align: center;
}

.brist {
  padding-bottom: 1rem;
  margin: 0;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);

}

@media (max-width: 1024px) {

  .main {
    grid-template-columns: 100%;
  }

  .left-panel {
    width: 100%;
    margin: 0;
  }

  #bristPanel {
    order: -1;
    min-height: 30rem;
  }

  .main {
    padding: 3rem 1rem;
  }

  button {
    padding: 0.5rem 1rem;
  }

  input {
    max-width: 150px;
  }

  .filter-bar {
    padding: 0.7rem 0.8rem;
  }
}

</style>
