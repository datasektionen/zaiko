<template>
  <div>
    <p>{{ item.name }}</p>
    <a target="_blank" :href="item.link">{{ item.link }}</a>
  </div>
</template>

<script setup lang="ts">
import type { Supplier } from '@/types';

defineProps<{
  item: Supplier
}>();

</script>

<style scoped>
* {
  font-family: Lato;
}

div {
  display: grid;
  grid-template-columns: 1fr 1fr;
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
</style>
