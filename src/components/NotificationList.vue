<template>
    <Teleport to="#noti">
        <div class="notiList">
          <NotificationPopup v-for="noti in notificationsStore.notifications" :notification="noti" :key="noti.id" @close="notificationsStore.remove(noti.id)"/>
        </div>
    </Teleport>
</template>

<script setup lang="ts">
import { useNotificationsStore } from "@/stores/notifications";
import NotificationPopup from "@/components/NotificationPopup.vue";

const notificationsStore = useNotificationsStore();
</script>


<style scoped>
.notiList {
  position: absolute;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  top: 6rem;
  right: 2rem;
  z-index: 8;
  border-radius: 2px;
}
</style>
