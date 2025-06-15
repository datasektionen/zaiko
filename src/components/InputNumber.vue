<template>
  <div>
    <div class="item">
      <div class="itemHeader">
        <component :is="icon" class="buttonIcon" />
        <p>{{ name }}</p>
      </div>
      <input type="number" v-model.number="model" :placeholder="name" :required :disabled="clubPerm != 'rw'">
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, type FunctionalComponent } from 'vue'

const props = defineProps<{
  name: string,
  icon: FunctionalComponent,
  clubPerm: string,
  required?: boolean,
}>()

const model = defineModel<number>();

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
