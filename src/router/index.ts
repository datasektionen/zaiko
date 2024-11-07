import { createRouter, createWebHistory } from 'vue-router'
import AddItem from '../views/AddItem.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/add',
      name: 'addItem',
      component: AddItem,
    },
  ],
})

export default router
