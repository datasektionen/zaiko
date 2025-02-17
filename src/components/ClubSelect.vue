<template>
  <div class="ClubSelect">
    <select class="dropdown" v-model="club">
      <option v-for="club in clubs" :key="club" :value="club">{{ club }}</option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, } from 'vue'

const club = ref<string>("")
const clubs = ref<Array<string>>([])

const GetClub = async () => {
  clubs.value = ["metadorerna", "mornarna", "metadorna", "gelo"]
}
GetClub()

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
