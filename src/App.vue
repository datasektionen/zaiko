<template>
  <BarHeader>
    <main>
      <RouterView />
    </main>
    <div id="popup"></div>
  </BarHeader>
  <NotificationList />
</template>

<script setup lang="ts">
import NotificationList from '@/components/NotificationList.vue';
import { RouterView } from 'vue-router';
import BarHeader from '@/components/BarHeader.vue';

</script>

<style scoped>
main {
  max-width: 1440px;
  margin: auto;
  position: relative;
  z-index: 0;
}
</style>
