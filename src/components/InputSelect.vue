<template>
  <div>
    <div class="item">
      <div class="itemHeader">
        <component :is="icon" class="buttonIcon" />
        <p>{{ name }}</p>
      </div>
      <select v-model="model" :placeholder="name" v-if="clubPerm == 'rw'">
        <option selected disabled>{{ name }}</option>
        <option value="Ingen">Ingen</option>
        <option v-for="item in items" :key="item.id" :value="itemVal(item)" :selected="current == item.name">{{ item.name }}
        </option>
      </select>
      <p v-else>{{ model }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, defineProps, type FunctionalComponent } from 'vue'

const props = defineProps<{
  name: string,
  icon: FunctionalComponent,
  clubPerm: string,
  current?: string,
  items: Array<any>,
}>()

const model = defineModel<string|number>()


const itemVal = (item: any): string|number => {
  // if model is a string return item name otherwise return id
  if (typeof model.value === 'string') {
    return item.name;
  } else {
    return item.id;
  }
}

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
