<template>
  <div @click="emit('close')" class="notiMain">
    <div>
      <img src="@/assets/check.svg" alt="check">
    </div>
    <div>
      <h6>{{ notification.title }}</h6>
      <p>{{ notification.message }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Notification } from '@/types';

defineProps<{
  notification: Notification
}>()

const emit = defineEmits(["close"]);

</script>

<style scoped>
.notiMain {
  background-color: #FAFAFA;
  display: flex;
  flex-direction: row;
}
</style>
