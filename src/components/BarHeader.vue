<template>
  <div :class="Main">
    <div :class="barSidePanel">
      <RouterLink to="/" class="titleDiv" @click="isMobileClose()">
        <svg width="42" height="42" viewBox="0 0 132.29167 132.29167" version="1.1" id="svg1" xml:space="preserve"
          xmlns="http://www.w3.org/2000/svg" xmlns:svg="http://www.w3.org/2000/svg" class="hover:animate-spin">
          <defs id="defs1" />
          <g id="layer1">
            <path
              style="fill:#01AD99;fill-opacity:1;stroke:none;stroke-width:8.47663;stroke-linejoin:bevel;paint-order:stroke markers fill"
              d="m 66.683955,16.5 -15.438401,8.913365 a 1.4433757,1.4433757 90.000001 0 0 -10e-7,2.5 l 15.438402,8.913366 a 5.196152,5.196152 180 0 0 5.196152,0 l 15.438402,-8.913366 a 1.4433757,1.4433757 89.999999 0 0 -1e-6,-2.5 L 71.880107,16.5 a 5.1961525,5.1961525 0 0 0 -5.196152,0 z"
              id="path8" transform="matrix(1.4926938,0,0,1.6129583,-37.271028,-25.490947)" />
            <path
              style="fill:#01AD99;fill-opacity:1;stroke:none;stroke-width:8.47663;stroke-linejoin:bevel;paint-order:stroke markers fill"
              d="m 70.585356,43.584156 v 15.569305 a 1.4433757,1.4433757 30.000001 0 0 2.165064,1.25 l 15.438401,-8.913364 a 5.1961522,5.1961522 120 0 0 2.598076,-4.5 l 0,-15.569305 a 1.4433757,1.4433757 30.000001 0 0 -2.165064,-1.25 l -15.438401,8.913364 a 5.1961522,5.1961522 120 0 0 -2.598076,4.5 z"
              id="path10" transform="matrix(1.4926938,0,0,1.6129583,-37.271028,-25.490947)" />
            <path
              style="fill:#01AD99;fill-opacity:1;stroke:none;stroke-width:8.47663;stroke-linejoin:bevel;paint-order:stroke markers fill"
              d="m 67.978707,43.584157 v 15.569305 a 1.4433759,1.4433759 150 0 1 -2.165064,1.25 L 50.375239,51.490098 a 5.1961518,5.1961518 59.999997 0 1 -2.598076,-4.5 V 31.420793 a 1.4433759,1.4433759 150 0 1 2.165064,-1.25 l 15.438404,8.913364 a 5.1961518,5.1961518 59.999997 0 1 2.598076,4.5 z"
              id="path10-4" transform="matrix(1.4926938,0,0,1.6129583,-37.271028,-25.490947)" />
            <path
              style="fill:#01AD99;fill-opacity:1;stroke:none;stroke-width:8.47663;stroke-linejoin:bevel;paint-order:stroke markers fill"
              d="M 43.875762,53.725041 28.43736,62.638406 a 1.4433756,1.4433756 90.000001 0 0 -1e-6,2.5 l 15.438403,8.913366 a 5.196152,5.196152 180 0 0 5.196152,0 l 15.438401,-8.913366 a 1.4433758,1.4433758 89.999999 0 0 0,-2.5 L 49.071914,53.725041 a 5.1961525,5.1961525 1.2281142e-6 0 0 -5.196152,0 z"
              id="path8-9" transform="matrix(1.4926938,0,0,1.6129583,-37.271028,-25.490947)" />
            <path
              style="fill:#01AD99;fill-opacity:1;stroke:none;stroke-width:8.47663;stroke-linejoin:bevel;paint-order:stroke markers fill"
              d="m 47.777163,80.809197 0,15.569305 a 1.4433757,1.4433757 30.000001 0 0 2.165064,1.25 l 15.4384,-8.913364 a 5.1961523,5.1961523 120 0 0 2.598076,-4.5 V 68.645833 a 1.4433757,1.4433757 30.000001 0 0 -2.165064,-1.25 l -15.4384,8.913364 a 5.1961523,5.1961523 120 0 0 -2.598076,4.5 z"
              id="path10-3" transform="matrix(1.4926938,0,0,1.6129583,-37.271028,-25.490947)" />
            <path
              style="color:#000000;font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:medium;line-height:normal;font-family:sans-serif;font-variant-ligatures:normal;font-variant-position:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-alternates:normal;font-variant-east-asian:normal;font-feature-settings:normal;font-variation-settings:normal;text-indent:0;text-align:start;text-decoration-line:none;text-decoration-style:solid;text-decoration-color:#000000;letter-spacing:normal;word-spacing:normal;text-transform:none;writing-mode:lr-tb;direction:ltr;text-orientation:mixed;dominant-baseline:auto;baseline-shift:baseline;text-anchor:start;white-space:normal;shape-margin:0;inline-size:0;clip-rule:nonzero;display:inline;overflow:visible;visibility:visible;isolation:auto;mix-blend-mode:normal;color-interpolation-filters:linearRGB;vector-effect:none;fill:#01AD99;fill-opacity:1;fill-rule:nonzero;stroke:none;stroke-linecap:butt;stroke-linejoin:bevel;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1;paint-order:stroke markers fill;image-rendering:auto;shape-rendering:auto;text-rendering:auto;enable-background:accumulate;stop-color:#000000;stop-opacity:1"
              d="m 45.170514,80.809198 0,15.569305 a 1.4433759,1.4433759 150 0 1 -2.165064,1.25 L 27.567046,88.715139 a 5.1961518,5.1961518 59.999997 0 1 -2.598076,-4.5 V 68.645834 a 1.4433759,1.4433759 150 0 1 2.165064,-1.25 l 15.438404,8.913364 a 5.1961518,5.1961518 59.999997 0 1 2.598076,4.5 z"
              id="path10-4-1" transform="matrix(1.4926938,0,0,1.6129583,-37.271028,-25.490947)" />
            <path
              style="fill:#01AD99;fill-opacity:1;stroke:none;stroke-width:8.47663;stroke-linejoin:bevel;paint-order:stroke markers fill"
              d="m 89.492148,53.725041 -15.438402,8.913365 a 1.4433756,1.4433756 90.000001 0 0 -10e-7,2.5 l 15.438403,8.913366 a 5.1961515,5.1961515 180 0 0 5.196152,0 l 15.4384,-8.913366 a 1.4433761,1.4433761 89.999999 0 0 0,-2.5 L 94.6883,53.725041 a 5.196152,5.196152 3.6843433e-6 0 0 -5.196152,0 z"
              id="path8-9-5" transform="matrix(1.4926938,0,0,1.6129583,-37.271028,-25.490947)" />
            <path
              style="fill:#01AD99;fill-opacity:1;stroke:none;stroke-width:8.47663;stroke-linejoin:bevel;paint-order:stroke markers fill"
              d="m 93.393549,80.809197 0,15.569301 a 1.443376,1.443376 30.000005 0 0 2.165064,1.25 l 15.438397,-8.91336 a 5.1961513,5.1961513 120.00001 0 0 2.59808,-4.5 V 68.645833 a 1.4433757,1.4433757 30.000001 0 0 -2.16506,-1.25 l -15.438405,8.913364 a 5.1961522,5.1961522 120 0 0 -2.598076,4.5 z"
              id="path10-3-0" transform="matrix(1.4926938,0,0,1.6129583,-37.271028,-25.490947)" />
            <path
              style="fill:#01AD99;fill-opacity:1;stroke:none;stroke-width:8.47663;stroke-linejoin:bevel;paint-order:stroke markers fill"
              d="m 90.7869,80.809198 v 15.56931 a 1.4433755,1.4433755 150 0 1 -2.165063,1.25 L 73.183432,88.715139 a 5.1961529,5.1961529 60.000002 0 1 -2.598076,-4.5 V 68.645834 a 1.4433759,1.4433759 150 0 1 2.165064,-1.25 l 15.438404,8.913364 a 5.1961518,5.1961518 59.999997 0 1 2.598076,4.5 z"
              id="path10-4-1-2" transform="matrix(1.4926938,0,0,1.6129583,-37.271028,-25.490947)" />
          </g>
        </svg>
        <h1 v-if="barOpen">Zaiko</h1>
      </RouterLink>
      <div class="navLinks" @click="isMobileClose()">
        <NavLink to="/" title="Dashboard" :compact="!barOpen" :icon="CommandLineIcon" />
        <NavLink to="/items" title="Produkter" :compact="!barOpen" :icon="ArchiveBoxIcon" />
        <NavLink to="/storages" title="Lager" :compact="!barOpen" :icon="InboxIcon" />
        <NavLink to="/suppliers" title="Leverantörer" :compact="!barOpen" :icon="ShoppingCartIcon" v-if="permsStore.hasGroup()"/>
        <NavLink to="/stock" title="Inventera" :compact="!barOpen" :icon="ClipboardDocumentListIcon" v-if="permsStore.hasWriteAccess()"/>
        <NavLink to="/admin" title="Admin" :compact="!barOpen" :icon="WrenchIcon" v-if="permsStore.isAdmin()"/>
      </div>
    </div>
    <div class="barPanel">
      <div class="barHeader">
        <div class="pageHeader">
          <button class="hamIcon" @click="barOpen = !barOpen">
            <Bars3Icon class="icon" />
          </button>
          <h1>{{ $route.params.name || $route.name }}</h1>
        </div>
      </div>
      <div class="mainContent">
        <slot />
      </div>
      <NotificationList />
      <PopupModal />
    </div>
  </div>
</template>

<script setup lang="ts">
import NavLink from '@/components/NavLink.vue'
import { CommandLineIcon, Bars3Icon, ArchiveBoxIcon, ShoppingCartIcon, ClipboardDocumentListIcon, WrenchIcon, InboxIcon } from '@heroicons/vue/24/outline'
import { computed, ref } from 'vue';
import NotificationList from '@/components/NotificationList.vue';
import { useMediaQuery } from '@vueuse/core';
import PopupModal from './PopupModal.vue';
import { usePermsStore } from '@/stores/permissions';

const isMobile = useMediaQuery("(max-width: 768px)");
const isSmallScreen = useMediaQuery("(max-width: 1250px)");

const barOpen = ref<boolean>(isSmallScreen || isMobile.value ? true : false);

const permsStore = usePermsStore();

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
  background-color: var(--zaiko-bg-2);
  min-height: 100vh;
  max-height: 100vh;
  z-index: 2;
}

.MainClosed {
  grid-template-columns: 96px 1fr;
}

.mainContent {
  /* overflow-x: hidden; */
  overflow-y: scroll;
  background-color: var(--zaiko-bg-1);
  border-top-left-radius: 16px;
  min-height: 100%;
  max-height: 90vh;
  width: 100%;
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
  color: var(--zaiko-main-color);
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

.pageHeader h1 {
  font-size: 1.8rem;
  color: var(--zaiko-text);
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
  color: var(--zaiko-text);
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

  .pageHeader h1 {
    font-size: 1.4rem;
    color: var(--zaiko-text);
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
