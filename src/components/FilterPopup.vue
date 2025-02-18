<template>
  <div class="filterContainer">
    <button :class="filterClass" @click="isOpen = !isOpen">
      <FunnelIcon />
    </button>
    <form :class="filterClass + ' filterPanel'" v-if="isOpen"
      @submit.prevent="emit('search', filterType[0], searchText)">
      <div class="filterHeader">
        <FunnelIconSmall />
        <div class="colSelect">
          <component :is="filterType[1]" />
          <select v-model="filterType" required>
            <option :value="['Fält', NumberedListIcon]" selected disabled>Fält</option>
            <option v-for="rec in columns" :key="rec[0]" :value="rec">{{ rec[0] }}</option>
          </select>
        </div>
      </div>
      <div class="filterContent">
        <button type="submit" class="filterSearch">
          <MagnifyingGlassIcon />
        </button>
        <input type="text" placeholder="Filter" v-model="searchText" required />
        <button class="clearButton" @click="emit('clear')">
          <TrashIcon />
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, type FunctionalComponent } from 'vue';
import { FunnelIcon, MagnifyingGlassIcon } from '@heroicons/vue/24/outline';
import { FunnelIcon as FunnelIconSmall, NumberedListIcon, TrashIcon } from '@heroicons/vue/20/solid';

const { columns } = defineProps<{
  columns: Map<string, FunctionalComponent>;
}>()

const emit = defineEmits(["search", "clear"]);

const isOpen = ref(false);
const filterType = ref<[string, FunctionalComponent]>(["Fält", NumberedListIcon]);
const searchText = ref();


const filterClass = computed(() => {
  return isOpen.value ? 'filterSelected' : '';
});

</script>

<style scoped>
button {
  all: unset;
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 0.4rem;
  margin: 0;
  border: none;
  cursor: pointer;
  background-color: #80CBC3;
  border-radius: 8px;
  color: #FAFAFA;
  width: 32px;
  height: 32px;
}

.filterSearch {
  background-color: inherit;
}

.filterContainer {
  position: relative;
}

.filterSelected {
  box-shadow: 0 4px 4px rgba(0, 0, 0, 0.25);
}

.filterPanel {
  border-radius: 8px;
  position: absolute;
  top: 3rem;
  right: 0;
  background-color: #FAFAFA;
  width: 210px;
  height: 100px;
  z-index: 3;
  padding: 0.66rem;
}

.filterHeader {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.filterHeader svg {
  width: 21px;
  height: 21px;
  color: #DADADA;
}

.colSelect svg {
  color: rgba(0, 0, 0, 0.33);

}

.colSelect {
  display: flex;
  align-items: center;
  background-color: #DADADA;
  padding: 0.4rem 0.5rem;
  border-radius: 6px;
}

.colSelect select {
  all: unset;
  appearance: auto;
  border-radius: 6px;
  margin-left: 0.3rem;
  min-width: 110px;
}

.filterContent {
  display: grid;
  grid-template-columns: 1fr 3fr 1fr;
  align-items: center;
  width: 100%;
  padding: 0;
  padding-top: 0.4rem;
}

.filterContent svg {
  width: 24px;
  height: 24px;
  color: rgba(0, 0, 0, 0.33);
}

.filterContent input {
  all: unset;
  padding: 0.3rem;
  border-radius: 0;
  margin-left: 0.3rem;
  max-width: 100px;
}

.filterContent button {
  aspect-ratio: 1;
  padding: 0.1rem;
}

.clearButton {
  background-color: #F6CDD6;
  aspect-ratio: 1;
  padding: 0.1rem;
}

.clearButton svg {
  color: #FAFAFA;
  width: 24px;
  height: 24px;
}
</style>
