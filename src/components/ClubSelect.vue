<template>
  <div class="ClubSelect">
    <select class="dropdown" v-model="clubs.active" @change="clubsStore.setClub(clubs.active)">
      <option v-for="club in clubs.clubs" :key="club.name" :value="club">{{ clubName(club.name) }}</option>
    </select>
    <div class="permission">
      <PencilIcon class="icon" v-if="clubs.active.permission == 'rw'" />
      <EyeIcon class="icon" v-else />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useClubsStore } from '@/stores/clubs'
import { PencilIcon, EyeIcon } from '@heroicons/vue/24/outline'
import { useMediaQuery } from '@vueuse/core/index.cjs';

const clubsStore = useClubsStore();
const clubs = await clubsStore.getClub();
const isMobile = useMediaQuery("(max-width: 550px)");

const clubName = (name: string) => {
  if (isMobile.value) {
    return name.slice(0, 3).toUpperCase();
  }
  return name;
};

</script>

<style scoped>
.ClubSelect {
  position: relative;
  display: flex;
  gap: 0.5rem;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  background-color: #FAFAFA;
  padding: 0.6rem 0.8rem;

}

.dropdown {
  all: unset;
}

p {
  margin: 0.2rem;
}

.selected {
  margin: auto;
  padding: 0;
  font-weight: semi-bold;
}

.icon {
  width: 18px;
  aspect-ratio: 1;
}
</style>
