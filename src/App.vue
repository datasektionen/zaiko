<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <div id="methone-buffer"></div>
  <!-- <header> -->
  <!--     <nav> -->
  <!--       <RouterLink to="/">Home</RouterLink> -->
  <!--       <RouterLink to="/about">About</RouterLink> -->
  <!--     </nav> -->
  <!-- </header> -->

  <RouterView />
</template>

<style scoped>
#methone-buffer {
  height: 50px;
}
</style>
