<template>
  <div :class="color">
    <h5>{{ amount }}</h5>
    <h6>{{ title }}</h6>
  </div>
</template>

<script setup lang="ts">
import { computed, defineProps } from 'vue'

const props = defineProps({
  title: String,
  amount: String,
  good: Boolean,
})

const color = computed<string>(() => {
  return props.good ? 'good' : 'bad'
})

</script>

<style scoped>
div {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  aspect-ratio: 1;
  gap: 12px;
}

h5 {
  font-size: 2.5rem;
  font-weight: semi-bold;
  margin: 0;
}

h6 {
  font-size: 0.7rem;
  margin: 0;
  color: rgba(0, 0, 0, 0.66);
}

.good {
  background-color: #B2DFDB;
}

.bad {
  background-color: #F6CDD6;
}
</style>
