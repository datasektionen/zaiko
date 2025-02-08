<script setup lang="ts">
import NotificationList from '@/components/NotificationList.vue';
</script>

<template>
  <div id="methone-buffer"></div>
  <!-- <header> -->
  <!--     <nav> -->
  <!--       <RouterLink to="/">Home</RouterLink> -->
  <!--       <RouterLink to="/about">About</RouterLink> -->
  <!--     </nav> -->
  <!-- </header> -->
  <main>
    <RouterView />
    <NotificationList />
  </main>
</template>

<style scoped>
#methone-buffer {
  height: 50px;
}

main {
  max-width: 1280px;
  margin: auto;
  position: relative;
  overflow: hidden;
}
</style>
