<template>
  <div v-if="popupStore.current()">
    <Teleport to="#popup">
      <div class="modalBackground" @click="popupStore.pop()">
        <div class="modal" @click.stop="">
          <div class="modalHeader">
            <TitleBig :title="currentPopop.title" :icon="currentPopop.icon" />
            <button @click="popupStore.pop()" class="text-(--zaiko-text) rounded-full">
              <XMarkIcon/>
            </button>
          </div>
          <KeepAlive>
            <component :is="currentPopop.component" v-bind="currentPopop.props"/>
          </KeepAlive>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import TitleBig from '@/components/TitleBig.vue';
import { PencilSquareIcon, XMarkIcon } from '@heroicons/vue/24/outline';
import { usePopupStore } from '@/stores/popup';
import { ref, watch } from 'vue';

const popupStore = usePopupStore();

const currentPopop = ref(popupStore.current());
watch(popupStore.popups, (val) => {
  currentPopop.value = val[val.length - 1];
})
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
  z-index: 3;
  background-color: var(--zaiko-bg-2);
  border-radius: 12px;
  padding: 1rem 3rem;
  min-width: 400px;
  max-width: 600px;
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
