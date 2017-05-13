<template>
  <div id="app">
    <div class="container">
      <div class="header">
        <h1>
          <i class="fa fa-bolt" aria-hidden="true"></i>
          Powergrid
          <small></small>
        </h1>
      </div>

      <router-view></router-view>

    </div>
  </div>
</template>

<script>
export default {
  name: 'app'
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>
