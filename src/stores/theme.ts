import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useThemeStore = defineStore('theme', () => {
  const theme = ref<'light' | 'dark'>(initializeTheme())
  const darkModeColors = {
    '--zaiko-bg-0': '#0a0b10',
    '--zaiko-bg-1': '#13171f',
    '--zaiko-bg-2': '#1c212c',
    '--zaiko-text': '#b3b9c5',
    '--zaiko-text-lc': '#454754',
    '--zaiko-text-hc': '#ffffff',
    '--zaiko-main-color': '#01ad99',
    '--zaiko-bad-color': '#db065e',
    '--zaiko-warning-color': '#ffb300',
    '--zaiko-link-color': '#01aaff',
    '--zaiko-table-divider': '#b4c2e0',
  }
  const lightModeColors = {
    '--zaiko-bg-0': '#ffffff',
    '--zaiko-bg-1': '#f5f6fa',
    '--zaiko-bg-2': '#e8eaf1',
    '--zaiko-text': '#333333',
    '--zaiko-text-lc': '#676a7e',
    '--zaiko-text-hc': '#000000',
    '--zaiko-main-color': '#01ad99',
    '--zaiko-bad-color': '#db065e',
    '--zaiko-warning-color': '#ffb300',
    '--zaiko-link-color': '#01aaff',
    '--zaiko-table-divider': '#b4c2e0',
  }

  function initializeTheme(): 'light' | 'dark' {
    let theme: 'light' | 'dark'
    const storageTheme = localStorage.getItem('theme')
    if (storageTheme === 'light' || storageTheme === 'dark') {
      theme = storageTheme
    } else if (
      window.matchMedia &&
      window.matchMedia('(prefers-color-scheme: dark)').matches
    ) {
      theme = 'dark'
      localStorage.setItem('theme', theme)
    } else {
      theme = 'light'
      localStorage.setItem('theme', theme)
    }

    return theme
  }

  function loadTheme() {
    const colors = getThemeColors()
    for (const [key, value] of Object.entries(colors)) {
      document.documentElement.style.setProperty(key, value)
    }
  }

  function setTheme(newTheme: 'light' | 'dark') {
    theme.value = newTheme
    loadTheme()
  }

  function toggleTheme() {
    const newTheme = getTheme() === 'light' ? 'dark' : 'light'
    setTheme(newTheme)
    localStorage.setItem('theme', newTheme)
  }

  function getTheme() {
    return theme.value
  }

  function getThemeColors() {
    return theme.value === 'dark' ? darkModeColors : lightModeColors
  }

  return { theme, initializeTheme, setTheme, getTheme, toggleTheme, loadTheme }
})
