import { createRouter, createWebHistory } from 'vue-router'
import MainPage from '../views/MainPage.vue';
import SupplierPage from '../views/SupplierPage.vue';
import ItemsPage from '@/views/ItemsPage.vue';
import StockPage from '@/views/StockPage.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/items',
      name: 'Items',
      component: ItemsPage,
    },
    {
      path: '/',
      name: 'main page',
      component: MainPage,
    },
    {
      path: '/stock',
      name: 'take stock',
      component: StockPage
    },
    {
      path: '/suppliers',
      name: 'list suppliers',
      component: SupplierPage,
    },
  ],
})

export default router
