<script setup lang="ts">
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
  </main>
</template>

<style scoped>
#methone-buffer {
  height: 50px;
}

main {
  max-width: 1280px;
  margin: auto;
}
</style>
