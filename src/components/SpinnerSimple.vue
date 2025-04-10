<template>
  <div class="loaderContainer">
    <span :style="{ borderColor: color }" class="Loader"></span>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  color?: string
}>()
</script>

<style scoped>
.Loader {
  width: 48px;
  height: 48px;
  border-width: 5px;
  border-style: solid;
  border-bottom-color: transparent !important;
  border-radius: 50%;
  display: inline-block;
  box-sizing: border-box;
  animation: rotation 1s linear infinite;
}

@keyframes rotation {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}

.loaderContainer {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  min-height: 100px;
  aspect-ratio: 1;
}
</style>
