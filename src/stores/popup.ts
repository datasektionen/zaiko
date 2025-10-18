import type { PopupItem } from "@/types";
import { defineStore } from "pinia";
import { markRaw, ref, shallowRef, type Component, type FunctionalComponent } from "vue";

export const usePopupStore = defineStore('popup', () => {

  const popups = ref<Array<PopupItem>>([])

  function push(popup: PopupItem) {
    console.log("Popups", popups.value)
    popups.value.push({ component: markRaw(popup.component), props: popup.props, title: popup.title, icon: popup.icon, cb: popup.cb })
  }

  function pop() {
    const popped = popups.value.pop()
    console.log("Popping popup", popped)
    return popped
  }

  function current() {
    return popups.value[popups.value.length - 1]
  }

  function callCurrent(result?: any) {
    const currentPopup = current()
    if (currentPopup && currentPopup.cb) {
      currentPopup.cb(result)
    }
  }

  return { push, pop, current, popups, callCurrent }
});
