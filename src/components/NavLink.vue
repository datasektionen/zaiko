<template>
  <div>
    <RouterLink :to="to" class="flex align-center text-decoration-none gap-3 cursor-pointer text-(--zaiko-text)">
      <component :is="icon" class=" flex justify-center align-center w-8 h-8 transition-none" />
      <h3 :class="isCompact + 'm-0 text-lg md:text-xl font-medium opacity-100 transition-all'" v-if="!compact">
        {{ title }}
      </h3>
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { computed, defineProps, type FunctionalComponent } from 'vue'
import { RouterLink } from 'vue-router'

const props = defineProps<{
  to: string,
  title: string,
  compact?: boolean,
  icon: FunctionalComponent,
}>()

const isCompact = computed<string>(() => {
  return props.compact ? "opacity-0" : "";
})

</script>

<style scoped>

@media (max-width: 768px) {
  h3 {
    font-size: 1.2rem;
  }
}
</style>
