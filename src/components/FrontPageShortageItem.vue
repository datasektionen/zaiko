<template>
  <div>
    <p>{{ item.name }}</p>
    <p>{{ item.location }}</p>
    <p>{{ item.current }}</p>
    <p>{{ item.order }}</p>
  </div>
</template>

<script setup lang="ts">
import type { StockGetResponse } from '@/types';

defineProps<{
  item: StockGetResponse
}>()

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
}

p, a {
  text-align: center;
  text-overflow: ellipsis;
  margin: 0;
}
</style>
