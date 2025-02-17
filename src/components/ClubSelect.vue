<template>
  <div class="ClubSelect">
    <select class="dropdown" v-model="clubs.club">
      <option v-for="club in clubs.clubs" :key="club" :value="club">{{ club }}</option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { useClubsStore } from '@/stores/clubs'

const clubsStore = useClubsStore();
const clubs = clubsStore.clubs;

</script>

<style scoped>
.ClubSelect {
  position: relative;
  display: inline-block;
}

.dropdown {
  all: unset;
  border-radius: 6px;
  background-color: #FAFAFA;
  padding: 0.6rem 0.8rem;
}

p {
  margin: 0.2rem;
}

.selected {
  margin: 0;
  padding: 0;
  font-weight: semi-bold;
}
</style>
