import { createRouter, createWebHistory } from 'vue-router'
import AddItem from '../views/AddItem.vue'
import TakeStock from '../views/TakeStock.vue';
import MainPage from '../views/MainPage.vue';
import AddSupplier from '@/views/AddSupplier.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/add',
      name: 'addItem',
      component: AddItem,
    },
    {
      path: '/',
      name: 'main page',
      component: MainPage,
    },
    {
      path: '/stock',
      name: 'take stock',
      component: TakeStock
    },
    {
      path: '/addSupplier',
      name: 'add supplier',
      component: AddSupplier,
    },
  ],
})

export default router
