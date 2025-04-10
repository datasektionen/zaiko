import { createRouter, createWebHistory } from 'vue-router'
import MainPage from '@/views/MainPage.vue';
import SupplierPage from '@/views/SupplierPage.vue';
import ItemsPage from '@/views/ItemsPage.vue';
import StockPage from '@/views/StockPage.vue';

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
  ],
})

export default router
