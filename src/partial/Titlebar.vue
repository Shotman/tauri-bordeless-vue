<template>
<div @dblclick="toggleMaximize" class="drag-region titlebar">
  <div @click="minimize" class="titlebar-button" id="titlebar-minimize">
    <img
      src="https://api.iconify.design/mdi:window-minimize.svg"
      alt="minimize"
    />
  </div>
  <div v-if="maximized" @click="toggleMaximize" class="titlebar-button" id="titlebar-maximize">
    <img
      src="https://api.iconify.design/mdi:window-restore.svg"
      alt="maximize"
    />
  </div>
  <div v-else @click="toggleMaximize" class="titlebar-button" id="titlebar-restore">
    <img
      src="https://api.iconify.design/mdi:window-maximize.svg"
      alt="restore"
    />
  </div>
  <div @click="close" class="titlebar-button" id="titlebar-close">
    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </div>
</div>
</template>

<script>
import { appWindow } from '@Tauri/window'

export default {
  data () {
    return {
      maximized: false
    }
  },
  created () {
    window.addEventListener('resize', this.resizeEventHandler)
  },
  destroyed () {
    window.removeEventListener('resize', this.resizeEventHandler)
  },
  methods: {
    minimize () {
      appWindow.minimize()
    },
    toggleMaximize () {
      this.maximized ? appWindow.unmaximize() : appWindow.maximize()
      this.maximized = !this.maximized
    },
    close () {
      appWindow.close()
    },
    resizeEventHandler () {
      if (window.screenX !== 0 && window.screenY !== 0 && this.maximized) {
        this.maximized = false
      }
    }
  }

}
</script>

<style scoped>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #5bbec3;
}

</style>
