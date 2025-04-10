<template>
  <div :class="Main">
    <div :class="barSidePanel">
      <RouterLink to="/" class="titleDiv" @click="isMobileClose()">
        <svg id="superdelta_img" width="26px" height="37px" viewBox="0 0 26 37" version="1.1"
          xmlns="http://www.w3.org/2000/svg">
          <g id="delta_logo" stroke-width="0.75" transform="translate(-19.000000, -14.000000)">
            <path
              d="M43.7787458,33.592285 C42.6511301,25.8619776 36.4053307,24.9682463 34.9922796,24.5299077 C32.3456685,23.7086951 27.8894062,23.9803328 27.4482787,20.1247049 C26.86666,15.0454778 30.7276382,17.5429168 33.2996277,17.7418823 C35.8708494,17.9408478 36.0983993,15 36.0983993,15 C36.0983993,15 35.4851508,16.5550034 30.2855895,15.2896138 C25.0858747,14.0243779 24.4670987,18.2920717 24.4242603,19.3975207 C24.3005051,22.5931057 26.3630408,24.1203001 26.5793821,24.9886806 C26.6736571,25.3657163 19.4190838,26.7772187 20.0373992,36.3168058 C20.6555611,45.8573148 26.5135123,49.7624152 31.054223,49.9901116 C35.5953943,50.2181153 45.6108143,46.5551535 43.7787458,33.592285 L43.7787458,33.592285 Z M40.3377062,39.8597738 C36.6622072,53.7530938 25.7316742,50.3181357 23.0692483,40.126495 C20.4789873,30.2109477 27.5310381,25.431782 27.5310381,25.431782 C27.5310381,25.431782 29.0851947,26.0448107 30.8936176,26.5548998 C31.9510644,26.8531175 43.4696649,28.0210985 40.3377062,39.8597738 L40.3377062,39.8597738 Z">
            </path>
          </g>
        </svg>
        <h1 v-if="barOpen">Zaiko</h1>
      </RouterLink>
      <div class="navLinks" @click="isMobileClose()">
        <NavLink to="/" title="Dashboard" :compact="!barOpen">
          <CommandLineIcon />
        </NavLink>
        <NavLink to="/items" title="Produkter" :compact="!barOpen">
          <ArchiveBoxIcon />
        </NavLink>
        <div v-if="clubs" class="navLinks">
          <NavLink to="/suppliers" title="Leverantörer" :compact="!barOpen" v-if="clubs.active.permission === 'rw'">
            <ShoppingCartIcon />
          </NavLink>
          <NavLink to="/stock" title="Inventera" :compact="!barOpen" v-if="clubs.active.permission === 'rw'">
            <ClipboardDocumentListIcon />
          </NavLink>
        </div>
      </div>
    </div>
    <div class="barPanel">
      <div class="barHeader">
        <div class="pageHeader">
          <button class="hamIcon" @click="barOpen = !barOpen">
            <Bars3Icon class="icon" />
          </button>
          <h1>{{ $route.name }}</h1>
        </div>
        <Suspense>
          <div class="clubMenu">
            <ArrowsUpDownIcon class="icon" v-if="!isMobile" />
            <ClubSelect />
          </div>
        </Suspense>
      </div>
      <div class="mainContent">
        <slot />
      </div>
      <NotificationList />
    </div>
  </div>
</template>

<script setup lang="ts">
import NavLink from '@/components/NavLink.vue'
import ClubSelect from '@/components/ClubSelect.vue'
import { ArrowsUpDownIcon, CommandLineIcon, Bars3Icon, ArchiveBoxIcon, ShoppingCartIcon, ClipboardDocumentListIcon } from '@heroicons/vue/24/outline'
import { computed, ref } from 'vue';
import NotificationList from '@/components/NotificationList.vue';
import { useMediaQuery } from '@vueuse/core/index.cjs';
import { useClubsStore } from '@/stores/clubs';
import type { ClubStorage } from '@/types';

const clubStore = useClubsStore();
const clubs = ref<ClubStorage | null>(null);
clubStore.getClub().then((club) => {
  clubs.value = club;
});

const isMobile = useMediaQuery("(max-width: 768px)");
const isSmallScreen = useMediaQuery("(max-width: 1250px)");

const barOpen = ref<boolean>(isSmallScreen || isMobile.value ? true : false);

const barSidePanel = computed<string>(() => {
  return barOpen.value ? "barSidePanel" : "barSidePanel closed";
});

const Main = computed<string>(() => {
  return barOpen.value ? "Main" : "Main MainClosed";
});

const isMobileClose = () => {
  if (isMobile.value) {
    barOpen.value = false;
  }
}

</script>

<style scoped>
.Main {
  display: grid;
  grid-template-columns: 220px 1fr;
  background-color: #DADADA;
  min-height: 100vh;
  max-height: 100vh;
  z-index: 2;
}

.MainClosed {
  grid-template-columns: 96px 1fr;
}

.mainContent {
  overflow-x: hidden;
  overflow-y: scroll;
  background-color: #FAFAFA;
  border-top-left-radius: 16px;
  min-height: 100%;
  max-height: 90vh;
}

.closed {
  width: 64px;
}

.barSidePanel {
  padding-left: 2rem;
  padding-top: 1rem;
  display: flex;
  flex-direction: column;
  gap: 4rem;
  max-width: 256px;
  overflow: hidden;
}

.titleDiv h1,
a {
  font-size: 1.8rem;
  font-weight: bold;
  text-decoration: none;
  color: black;
}

.titleDiv {
  display: flex;
  align-items: center;
  gap: 16px;
}

.navLinks {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  transition: all 0.3s ease-in;
}

.barHeader {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-right: 2rem;
}

.barPanel {
  min-width: 100%;
  display: grid;
  grid-template-rows: 64px 1fr;
}

.pageHeader {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  gap: 16px;
}

.clubMenu {
  display: flex;
  align-items: center;
  gap: 12px;
}

.hamIcon {
  all: unset;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon {
  width: 34px;
  height: 34px;
}

@media (max-width: 890px) {
  .Main {
    grid-template-columns: 210px 1fr;
  }

  .closed {
    width: 0;
  }

  .MainClosed {
    grid-template-columns: 0 1fr;
  }

  .barHeader {
    padding-right: 1rem;
  }

  .barHeader h1 {
    font-size: 1.4rem;
  }

  .barPanel {
    grid-template-rows: 56px 1fr;
  }
}

@media only screen and (max-width: 768px) {
  .Main {
    overflow: hidden;
  }

  .MainClosed {
    overflow: initial;
  }
}
</style>
