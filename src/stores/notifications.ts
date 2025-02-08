import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { Notification } from "@/types";

export const useNotificationsStore = defineStore('notifications', () => {
  const notifications = ref<Array<Notification>>([])

  function add(notification: Notification) {
    notifications.value.push(notification)
    console.log('notifications', notifications.value)
  }

  function pop() {
    notifications.value.pop()
  }

  function remove(id: number) {
    notifications.value = notifications.value.filter((notification) => notification.id !== id)
  }

  return { notifications, add, pop, remove }
})
