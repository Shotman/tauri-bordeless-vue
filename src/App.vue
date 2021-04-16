<template>
  <div id="container">
      <header>
        <TitleBar />
        <p>top</p>
      </header>
      <main>
        <section>1st</section>
        <section>2nd</section>
        <section>3rd<br>
          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.<br>
          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.<br>
          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.<br>
        </section>
        <section>4th</section>
        <section>5th</section>
      </main>
      <footer>bottom</footer>
  </div>
</template>

<script>
import TitleBar from './partial/Titlebar'
export default {
  name: 'App',
  components: {
    TitleBar
  }
}
</script>

<style lang="scss">
#container {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  display: flex; /* displays flex-items (children) inline */
  flex-direction: column; /* stacks them vertically */
  height: 100%; /* needs to take the parents height, alternative: body {display: flex} */
}

main {
  flex: 1; /* takes the remaining height of the "container" div */
  overflow: auto; /* to scroll just the "main" div */
  background: lightgreen;
}

section {
  height: 100%;
  overflow: auto; /* recommended */
  flex: 1;
  border-bottom: 1px solid;
  background: lightgreen;
}

header {background: #f88}
section:last-child {border: none}
footer {background: lightblue}
</style>
