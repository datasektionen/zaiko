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

  return { notifications, add, pop }
})
