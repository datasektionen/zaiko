<template>
  <div class="titleDiv">
    <div class="icon">
      <slot />
    </div>
    <h2>{{ title }}</h2>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'

defineProps<{
  title: string,
}>()

</script>

<style scoped>
.titleDiv {
  display: flex;
  align-items: center;
  gap: 7px;
  margin: 1rem;
  margin-left: 0;
  padding: 3px 0;
  padding-right: 12px;
  border-bottom: 2px solid #26A69A;
  width: fit-content;
  max-height: 55px;
}

.icon {
  display: flex;
  justify-content: center;
  align-items: center;
  min-width: 24px;
  min-height: 24px;
}

h2 {
  margin: 0;
  font-size: 1.4rem;
  font-weight: semi-bold;
}

@media (max-width: 520px) {
  .titleDiv {
    margin: 0;
    margin-bottom: 1rem;
    padding: 0;
  }
}

</style>
