import { createRouter, createWebHistory } from 'vue-router'
import MainPage from '@/views/MainPage.vue'
import SupplierPage from '@/views/SupplierPage.vue'
import ItemsPage from '@/views/ItemsPage.vue'
import ItemPage from '@/views/ItemPage.vue'
import StockPage from '@/views/StockPage.vue'
import AdminPage from '@/views/AdminPage.vue'
import { usePermsStore } from '@/stores/permissions'
import StoragesPage from '@/views/StoragesPage.vue'
import StoragePage from '@/views/StoragePage.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'Dashboard',
      component: MainPage,
    },
    {
      path: '/items',
      name: 'Produkter',
      component: ItemsPage,
    },
    {
      path: '/item/:name',
      name: 'Produkt',
      component: ItemPage,
    },
    {
      path: '/storages',
      name: 'Lager',
      component: StoragesPage,
    },
    {
      path: '/storage/:name',
      name: 'lager',
      component: StoragePage,
    },
    {
      path: '/suppliers',
      name: 'Leverantörer',
      component: SupplierPage,
    },
    {
      path: '/stock',
      name: 'Inventering',
      component: StockPage,
      beforeEnter: (to, from, next) => {
        const permsStore = usePermsStore()
        if (permsStore.hasWriteAccess() || permsStore.isAdmin()) {
          next()
        } else {
          next('/')
        }
      },
    },
    {
      path: '/admin',
      name: 'Admin',
      component: AdminPage,
      beforeEnter: (to, from, next) => {
        const permsStore = usePermsStore()
        if (permsStore.isAdmin()) {
          next()
        } else {
          next('/')
        }
      },
    },
  ],
})

router.beforeEach((to, from, next) => {
  const permsStore = usePermsStore()
  if (permsStore.perms === undefined) {
    permsStore
      .fetchPermissions()
      .then(() => {
        next()
      })
      .catch(() => {
        next()
      })
  } else {
    next()
  }
})

export default router
