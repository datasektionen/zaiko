<template>
  <div>
    <div class="item">
      <div class="itemHeader">
        <component :is="icon" class="buttonIcon" />
        <p>{{ name }}</p>
      </div>
      <select
        class="disabled:opacity-35"
        v-model="model"
        :placeholder="name"
        :required="required"
        :disabled="disabled"
      >
        <slot :row="row" v-for="(row, idx) in items" :key="idx" name="row" />
      </select>
    </div>
  </div>
</template>

<script setup lang="ts" generic="T extends Record<string, any> | string">
import { defineProps, type FunctionalComponent } from 'vue'

const props = defineProps<{
  name: string
  icon: FunctionalComponent
  required?: boolean
  disabled?: boolean
  items: Array<T>
}>()

const model = defineModel<string | number>()
</script>

<style scoped>
.buttonIcon {
  width: 1.5rem;
  height: 1.5rem;
}

.itemHeader {
  display: flex;
  align-items: center;
  gap: 4px;
}

.itemHeader svg {
  color: var(--zaiko-text);
}

.itemHeader p {
  margin: 0;
  font-weight: 500;
  color: var(--zaiko-text);
}

.item {
  display: flex;
  justify-content: space-between;
  flex-direction: column;
  gap: 0.5rem;
  padding: 8px 0;
  margin-bottom: 8px;
  max-width: 100%;
}

.item select {
  border: none;
  background-color: inherit;
  appearance: auto;
  width: 100%;
  border: 1px solid var(--zaiko-text);
  padding: 0.5rem;
  border-radius: 8px;
  color: var(--zaiko-text);
  font-size: 16px;
}
</style>
