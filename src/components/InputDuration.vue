<template>
  <div>
    <div class="item">
      <div class="itemHeader">
        <component :is="icon" class="buttonIcon" />
        <p>{{ name }}</p>
      </div>
      <div :class="'grid grid-flow-col gap-2 items-center' + (disabled ? ' opacity-50' : '')">
        <span class="flex items-center gap-1 border-1 border-(--zaiko-text) rounded-lg p-1">
          <p class="text-(--zaiko-text) text-lg">År:</p>
          <input type="number" v-model="model!.years" placeholder="år" :required min="0" :disabled="disabled">
        </span>
        <span class="flex items-center gap-1 border-1 border-(--zaiko-text) rounded-lg p-1">
          <p class="text-(--zaiko-text) text-lg">Månader:</p>
          <input type="number" v-model="model!.months" placeholder="månader" :required min="0" max="11" :disabled="disabled">
        </span>
        <span class="flex items-center gap-1 border-1 border-(--zaiko-text) rounded-lg p-1">
          <p class="text-(--zaiko-text) text-lg">Dagar:</p>
          <input type="number" v-model="model!.days" placeholder="dagar" :required min="0" max="30" :disabled="disabled">
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Duration } from '@/types';
import { defineProps, type FunctionalComponent } from 'vue'

const props = defineProps<{
  name: string,
  icon: FunctionalComponent,
  required?: boolean,
  disabled?: boolean,
}>()

const model = defineModel<Duration>()

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
  font-size: 18px;
  color: var(--zaiko-text);
  border: none;
  background-color: inherit;
  width: 100%;
  padding: 0.5rem;
}
</style>
