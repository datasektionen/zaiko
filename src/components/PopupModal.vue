<template>
  <div v-if="modal">
    <Teleport to="#popup">
      <div class="modalBackground" @click="emit('exit')">
        <div class="modal" @click.stop="">
          <div class="modalHeader">
            <TitleBig :title="title">
              <PencilSquareIcon />
            </TitleBig>
            <button @click="emit('exit')">
              <XMarkIcon />
            </button>
          </div>
          <slot />
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import TitleBig from '@/components/TitleBig.vue';
import { PencilSquareIcon, XMarkIcon } from '@heroicons/vue/24/outline';


defineProps<{
  modal: boolean
  title: string
}>()

const emit = defineEmits(["exit"]);
</script>

<style scoped>
.modalBackground {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 2;
}

.modal {
  width: 550px;
  z-index: 3;
  background-color: var(--zaiko-bg-2);
  border-radius: 12px;
  padding: 1rem 3rem;
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
  color: var(--zaiko-text);
}

@media (max-width: 768px) {
  .modal {
    padding: 15px 1rem;
    max-width: 95vw;
    min-width: unset;
    top: 5rem;
  }

  .modalHeader {
    gap: 1rem;
  }
}
</style>
