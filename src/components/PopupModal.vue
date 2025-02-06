<template>
  <div v-if="modal">
    <Teleport to="#modal">
      <div class="modal-bg" @click="emit('exit')">
        <div class="modal" @click.stop="">
          <slot />
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  modal: boolean
}>()

const emit = defineEmits([ "exit" ]);
</script>

<style scoped>
.modal {
  position: relative;
  display: block;
  max-width: 660px;
  z-index: 11;
  background-color: #f1f1f1;
  border-radius: 2px;
  padding: 15px 3rem;
  min-height: 80vh;
}

.modal-bg {
  position: absolute;
  z-index: 10;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: rgba(0, 0, 0, 0.20);
}
</style>
