<template>
  <div>
    <p>{{ item.name }}</p>
    <p>{{ item.supplier }}</p>
    <p>{{ item.current }}</p>
    <input type="number" v-model.number="input" />
    <p :class="diffClass">{{ input - item.current }}</p>
  </div>
</template>

<script setup lang="ts">
import type { Item } from '@/types';
import { computed } from 'vue';

const props = defineProps<{ item: Item }>()

const input = defineModel<number>({ default: 0 });

const diffClass = computed(() => {
  const diff = input.value - props.item.current
  if (diff >= 0) {
    return "Green"
  } else {
    return "Red"
  }
})

</script>

<style scoped>
* {
  font-family: Lato;
}

div {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  padding: 10px 0;
  background-color: rgba(0, 0, 0, 0);
}

p,
a {
  text-align: center;
  text-overflow: ellipsis;
  margin: 0;
}

input {
  width: 50px;
  text-align: center;
  margin: auto;
  border: none;
  background-color: inherit;
}

.Red {
  color: red;
}

.Green {
  color: green;
}
</style>
