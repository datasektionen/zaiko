import { createRouter, createWebHistory } from 'vue-router'
import MainPage from '@/views/MainPage.vue';
import SupplierPage from '@/views/SupplierPage.vue';
import ItemsPage from '@/views/ItemsPage.vue';
import StockPage from '@/views/StockPage.vue';
import ImportPage from '@/views/ImportPage.vue';

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
      path: '/suppliers',
      name: 'Leverantörer',
      component: SupplierPage,
    },
    {
      path: '/stock',
      name: 'Inventering',
      component: StockPage,
    },
    {
      path: '/import',
      name: 'Importera',
      component: ImportPage,
    },
  ],
})

export default router
