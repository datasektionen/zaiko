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
  gap: 12px;
  margin: 1rem;
  padding: 4px 0;
  padding-right: 8px;
  border-bottom: 2px solid #26A69A;
  width: fit-content;
}

.icon {
  display: flex;
  justify-content: center;
  align-items: center;
  min-width: 30px;
  min-height: 30px;
}

h2 {
  margin: 0;
  font-size: 1.8rem;
  font-weight: semi-bold;
}

</style>
