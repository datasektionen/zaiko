<template>
  <div>
    <RouterLink :to="to">
      <div class="icon">
        <slot />
      </div>
      <h3 :class="isCompact" v-if="!compact">
        {{ title }}
      </h3>
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { computed, defineProps } from 'vue'
import { RouterLink } from 'vue-router'

const props = defineProps<{
  to: string,
  title: string,
  compact?: boolean,
}>()

const isCompact = computed<string>(() => {
  return props.compact ? "compact" : "";
})

</script>

<style scoped>
a {
  display: flex;
  align-items: center;
  text-decoration: none;
  color: black;
  gap: 12px;
  cursor: pointer;
  text-overflow: ellipsis;
}

.icon {
  display: flex;
  justify-content: center;
  align-items: center;
  min-width: 32px;
  min-height: 32px;
  transition: none;
}

.compact {
  opacity: 0;
}

h3 {
  margin: 0;
  font-size: 1.2rem;
  font-weight: medium;
  opacity: 1;
  transition: all 0.3s ease-in;
}

@media (max-width: 768px) {
  h3 {
    font-size: 1.2rem;
  }
}
</style>
