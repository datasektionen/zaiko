<template>
  <div v-if="modal">
    <Teleport to="#club">
      <div class="modal-bg" @click="emit('exit')">
        <div class="modal" @click.stop="">
          <h2>Nämnder</h2>
          <div class="clubContent">
          <div class="club" v-for="club in clubsStore.clubs.clubs" :key="club" @click="Done(club)">
            <p>{{ club }}</p>
          </div>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { useClubsStore } from '@/stores/clubs';

defineProps<{
  modal: boolean
}>()

const emit = defineEmits(["exit"]);

const clubsStore = useClubsStore();

const Done = (club: string) => {
  clubsStore.setClub(club);
  emit('exit');
};
</script>

<style scoped>
.modal {
  position: relative;
  display: block;
  max-width: 660px;
  min-width: 200px;
  z-index: 11;
  background-color: #f1f1f1;
  border-radius: 2px;
  padding: 15px 3rem;
}

.modal-bg {
  position: absolute;
  z-index: 10;
  top: 48px;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: rgba(0, 0, 0, 0.20);
}

h2 {
  padding-bottom: 10px;
  margin: 0;
  border-bottom: 2px solid rgba(0, 105, 92, 0.25);
  width: auto;
  font-size: 2.6rem;
}

.club {
  padding: 8px 0;
  cursor: pointer;
  min-width: 150px;
  border-radius: 2px;
}

.club p {
  margin: 0;
  font-size: 1.8rem;
  text-align: center;
}

.club:hover {
  background-color: rgba(0, 105, 92, 0.25);
  cursor: pointer;
}

.clubContent {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: flex-start;
  margin: 2rem auto;
  max-height: 25vh;
  overflow-y: scroll;
}
</style>
