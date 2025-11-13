<template>
  <div>
    <div class="item">
      <div class="itemHeader">
        <component :is="icon" class="buttonIcon" />
        <p>{{ name }}</p>
      </div>
      <div class="flex items-center">
        <input
          type="checkbox"
          v-model="model"
          :required
          class="w-8 h-8 bg-(--zaiko-bg-2) border-2 text-(--zaiko-main-color)"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, type FunctionalComponent } from 'vue'

const props = defineProps<{
  name: string
  icon: FunctionalComponent
  required?: boolean
}>()

const model = defineModel<boolean>()
</script>

<style scoped>
input {
  font-size: 16px;
  color: var(--zaiko-text);
  accent-color: var(--zaiko-main-color);
  background-color: var(--zaiko-bg-2);
}

.buttonIcon {
  width: 1.5rem;
  height: 1.5rem;
}

.itemHeader {
  display: flex;
  align-items: center;
  gap: 4px;
}

.itemHeader p {
  margin: 0;
  font-weight: 500;
  color: var(--zaiko-text);
}

.itemHeader svg {
  color: var(--zaiko-text);
}

.item {
  display: flex;
  justify-content: space-between;
  gap: 0.5rem;
  padding: 8px 0;
  max-width: 100%;
}

.item input {
  border: none;
  background-color: inherit;
  width: 100%;
  border: 1px solid var(--zaiko-text);
  padding: 0.5rem;
  border-radius: 8px;
}
</style>
