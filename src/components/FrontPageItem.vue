<template>
  <div>
    <a target="_blank" :href="item.link" v-if="item.link">{{ item.name }}</a>
    <p v-else>{{ item.name }}</p>
    <p>{{ item.location }}</p>
    <p>{{ item.current }}</p>
    <p>{{ status }}</p>
  </div>
</template>

<script setup lang="ts">
import type { Item } from '@/types';
import { computed } from 'vue';

const props = defineProps<{
  item: Item
}>()

const status = computed<string>(() => {
  const min = props.item.min ? props.item.min : 0
  const diff = props.item.current - min

  if (diff > 3) {
    return '✅'
  } if (diff > 0) {
    return '⚠️'
  } else {
    return '🛑'
  }
});

</script>

<style scoped>
* {
  font-family: Lato;
}

div {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  padding: 10px 0;
  background-color: rgba(0,0,0,0);
}

p, a {
  text-align: center;
  text-overflow: ellipsis;
  margin: 0;
}
</style>
