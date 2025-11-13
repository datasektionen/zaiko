<template>
  <div>
    <div class="item">
      <div class="itemHeader">
        <component :is="icon" class="buttonIcon" />
        <p>{{ name }}</p>
      </div>
      <input
        v-model="model"
        :placeholder="placeholder || name"
        :required
        minlength="1"
        :maxlength="maxlength || 100"
        :disabled="disabled"
        class="disabled:opacity-50"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, type FunctionalComponent } from 'vue'

const props = defineProps<{
  name: string
  icon: FunctionalComponent
  required?: boolean
  placeholder?: string
  disabled?: boolean
  maxlength?: number
}>()

const model = defineModel<string>()
</script>

<style scoped>
input {
  font-size: 16px;
  color: var(--zaiko-text);
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
  flex-direction: column;
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
