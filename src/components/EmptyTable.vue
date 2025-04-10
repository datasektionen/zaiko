<template>
  <div class="shortageEmpty">
    <component :is="icon" />
    <h3 v-if="!compact">{{ text }}</h3>
  </div>
</template>

<script setup lang="ts">
import type { FunctionalComponent } from 'vue';

defineProps<{
  compact?: boolean,
  text: string,
  icon: FunctionalComponent,
}>()

</script>

<style scoped>
.shortageEmpty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  min-height: 7rem;
  color: #DADADA;
}

.shortageEmpty h3 {
  margin-left: 1rem;
}

.shortageEmpty svg {
  width: 3rem;
  aspect-ratio: 1;
}
</style>
