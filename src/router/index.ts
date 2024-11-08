import { createRouter, createWebHistory } from 'vue-router'
import AddItem from '../views/AddItem.vue'
import ListItems from '../views/ListItems.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/add',
      name: 'addItem',
      component: AddItem,
    },
    {
      path: '/items',
      name: 'list items',
      component: ListItems,
    }
  ],
})

export default router
