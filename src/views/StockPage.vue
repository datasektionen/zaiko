<template>
  <div class="main">
    <PanelTemplate title="Inventera" :icon="ClipboardDocumentListIcon">
    </PanelTemplate>
  </div>
</template>

<script setup lang="ts">
import PanelTemplate from '@/components/PanelTemplate.vue'
import { ClipboardDocumentListIcon, InboxIcon } from '@heroicons/vue/24/outline'
import { ArchiveBoxIcon, HomeIcon, ShoppingCartIcon, WalletIcon, InboxArrowDownIcon, ArrowsUpDownIcon, ClipboardDocumentListIcon as ClipboardDocumentListIconSolid } from '@heroicons/vue/16/solid'
import { ref } from 'vue'
import type { StorageTreeGetResponse } from '@/types'
import { getStorageTree } from '@/stores/storageData'

</script>

<style scoped>
</style>
