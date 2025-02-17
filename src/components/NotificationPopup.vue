<template>
  <div @click="emit('close')" class="notiMain" :class="getColor">
    <div>
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
        class="size-6" v-if="iconType == 0">
        <path stroke-linecap="round" stroke-linejoin="round"
          d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
      </svg>
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
        class="size-6" v-else>
        <path stroke-linecap="round" stroke-linejoin="round"
          d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
      </svg>
    </div>
    <div class="notiText">
      <h6>{{ notification.title }}</h6>
      <p>{{ notification.message }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Notification } from '@/types';
import { computed, onMounted } from 'vue';

const props = defineProps<{
  notification: Notification
}>()

const emit = defineEmits(["close"]);

onMounted(() => {
  setTimeout(() => {
    emit('close');
  }, 3000);
});

const getColor = computed<string>(() => {
  switch (props.notification.severity) {
    case "info":
      return "green";
    case "warning":
      return "yellow";
    case "error":
      return "red";
    default:
      return "green";
  }
});

const iconType = computed<number>(() => {
  switch (props.notification.severity) {
    case "info":
      return 3;
    case "warning":
      return 1;
    case "error":
      return 2;
    default:
      return 0;
  }
});

</script>

<style scoped>
.notiMain {
  background-color: #FAFAFA;
  display: flex;
  flex-direction: row;
  gap: 1rem;
  min-width: 290px;
  padding: 10px 8px;
  box-shadow: 0 4px 6px 1px rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  cursor: pointer;
  border-bottom: 3px solid;
}

svg {
  width: 52px;
  height: 52px;
}

.notiText h6 {
  font-size: 1.4rem;
  margin: 0;
}

.notiText p {
  font-size: 12px;
  margin-top: 0.5rem;
  color: rgba(0, 0, 0, 0.33);
  text-overflow: ellipsis;
}

.green {
  color: #2EB563;
}

.yellow {
  color: #b6b42e;
}

.red {
  color: #B62E3D;
}
</style>
