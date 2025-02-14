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
        <div :class="itemSelected(idx)" v-for="(item, idx) in items" :key="item.id" @click="SelectItem(idx)">
          <FrontPageItem :item="item" />
        </div>
      </div>
    </div>
    <div class="left-panel" id="selectPanel" v-if="items.length > 0 && selectedIndex && !isMobile">
      <ItemPanel :item="selectedIndex" :key="selectedIndex.id" />
    </div>
    <PopupModal :modal="openEdit" @exit="DoneEdit()" v-else-if="items.length > 0 && selectedIndex">
      <ItemPanel :item="selectedIndex" :key="selectedIndex.id" @updated="DoneEdit()"/>
    </PopupModal>
    <PopupModal :modal="openModal" @exit="DoneModal()">
      <AddForm @done="DoneModal()" />
    </PopupModal>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import FrontPageItem from '@/components/FrontPageItem.vue';
import PopupModal from '@/components/PopupModal.vue';
import AddForm from '@/components/AddForm.vue';
import ItemPanel from '@/components/ItemPanel.vue';
import type { ItemGetResponse, Notification } from '@/types';
import { useNotificationsStore } from '@/stores/notifications';
import { useMediaQuery } from '@vueuse/core/index.cjs';
import { useClubsStore } from '@/stores/clubs';
const HOST = import.meta.env.VITE_HOST;

const UpdateMethone = () => {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  window.methone_conf.update({
    login_href: "/items?club",
  });
};
window.onload = UpdateMethone;

const items = ref<Array<ItemGetResponse>>([]);
const notificationsStore = useNotificationsStore();
const clubStore = useClubsStore();

const GetData = () => {
  const url: string = HOST + "/api/" + clubStore.getClub();
  fetch(url + "/item", {
    method: "GET",
  }).then((r) => r.json()).then((r) => items.value = r)
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
GetData()

const DoneModal = () => {
  openModal.value = false;
  GetData()
}
const DoneEdit = () => {
  openEdit.value = false;
  selected.value = -1;
  GetData()
}

const openModal = ref<boolean>(false)
const openEdit = ref<boolean>(false)
const selected = ref<number>(-1);

const itemSelected = (id: number) => {
  if (id == selected.value) {
    return "item-selected"
  } else {
    return "item"
  }
}

const SelectItem = (id: number) => {
  if (!isMobile) {
    selected.value = id;
  } else {
    openEdit.value = true;
    selected.value = id;
  }
}

const selectedIndex = computed<ItemGetResponse>(() => {
  return items.value[selected.value];
})

const isMobile = useMediaQuery('(max-width: 1024px)');

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

.item {
  border-radius: 2px;
  cursor: pointer;
}

.item-selected {
  border-radius: 2px;
  background-color: rgba(0, 105, 92, 0.25);
  cursor: pointer;
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

@media (max-width: 1024px) {

  .main {
    grid-template-columns: 100%;
  }

  .left-panel {
    width: 100%;
    margin: 0;
  }

  #selectPanel {
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
