<template>
  <div>
    <button @click="notifyJS">Send Notification from JS</button>
    <button @click="notifyRust">Send Notification from Rust</button>
    <button @click="notifyNative">Send Notification from NativeJS</button>
  </div>
</template>

<script>
import { sendNotification, isPermissionGranted, requestPermission } from '@tauri-apps/api/notification'
import { invoke } from '@tauri-apps/api/tauri'
export default {
  methods: {
    notifyJS: () => {
      isPermissionGranted().then(granted => {
        if (!granted) {
          requestPermission().then(result => { if (result) { sendNotification('TOTO') } })
        } else {
          sendNotification('TOTO')
        }
      })
    },
    notifyRust: () => {
      invoke('notify')
    },
    notifyNative: () => {
      if (!('Notification' in window)) {
        alert('Ce navigateur ne prend pas en charge la notification de bureau')
      } else if (Notification.permission === 'granted') {
        Notification('Salut toi!')
      } else if (Notification.permission !== 'denied') {
        Notification.requestPermission().then((permission) => {
          if (permission === 'granted') {
            Notification('Salut toi!')
          }
        })
      }
    }
  }
}
</script>

<style>

</style>
