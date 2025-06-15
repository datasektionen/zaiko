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
  amount: Number,
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
  box-shadow: 0 4px 4px rgba(0, 0, 0, 0.1);
}

h5 {
  font-size: 2.5rem;
  font-weight: semi-bold;
  margin: 0;
  color: var(--zaiko-bg-1);
}

h6 {
  font-size: 0.7rem;
  margin: 0;
  color: var(--zaiko-bg-1);
}

.good {
  background-color: var(--zaiko-main-color);
}

.bad {
  background-color: var(--zaiko-bad-color);
}

@media (max-width: 768px) {
  div {
    aspect-ratio: 1.2;
  }

}
</style>
