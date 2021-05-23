<template>
  <div>
    <video controls id="video" src="http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"></video>
    <button @click="togglepip">Toggle PIP</button>
  </div>
</template>

<script>
export default {
  methods: {
    togglepip: () => {
      if (document.pictureInPictureElement) {
        document.exitPictureInPicture()
      } else {
        if (document.pictureInPictureEnabled) {
          document.getElementById('video').requestPictureInPicture()
        }
      }
    }
  }
}
</script>

<style lang="scss" scoped>
#video{
  max-width: 100%;
}
</style>
