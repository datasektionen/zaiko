<template>
  <div id="methone-buffer"></div>
  <main>
    <RouterView />
    <NotificationList />
    <ClubModal :modal="OpenModal" @exit="CloseModal()" />
  </main>
</template>

<script setup lang="ts">
import NotificationList from '@/components/NotificationList.vue';
import ClubModal from '@/components/ClubModal.vue';
import { ref } from 'vue';

const OpenModal = ref<boolean>(false);
const params = new URLSearchParams(window.location.search);

if (params.has('club')) {
  OpenModal.value = true;
}

const CloseModal = () => {
  OpenModal.value = false
};

</script>

<style scoped>
#methone-buffer {
  height: 50px;
}

main {
  max-width: 1280px;
  margin: auto;
  position: relative;
  overflow: scroll;
  overflow-x: hidden;
}
</style>
