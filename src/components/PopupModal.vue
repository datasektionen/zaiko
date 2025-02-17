<template>
  <div v-if="modal">
    <Teleport to="#popup">
      <div class="modal" @click.stop="">
        <div class="modalHeader">
          <TitleMedium :title="title">
            <PencilSquareIcon />
          </TitleMedium>
          <button @click="emit('exit')">
            <XMarkIcon />
          </button>
        </div>
        <slot />
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import TitleMedium from '@/components/TitleMedium.vue';
import { PencilSquareIcon, XMarkIcon } from '@heroicons/vue/24/outline';


defineProps<{
  modal: boolean
  title: string
}>()

const emit = defineEmits(["exit"]);
</script>

<style scoped>
.modal {
  position: absolute;
  top: 6rem;
  right: 0;
  width: 550px;
  z-index: 3;
  background-color: #FAFAFA;
  border-top-left-radius: 12px;
  border-bottom-left-radius: 12px;
  padding: 15px 3rem;
  min-height: 70vh;
  min-width: 500px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.25);
}

.modalHeader {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

button {
  all: unset;
  padding: 0.6rem;
  background-color: transparent;
  border: none;
  cursor: pointer;
}

.modalHeader button svg {
  width: 26px;
  height: 26px;
}

@media (max-width: 768px) {
  .modal {
    padding: 15px 1rem;
  }
}
</style>
